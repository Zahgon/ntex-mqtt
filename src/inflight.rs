
use std::{cell::Cell, fmt, future::poll_fn, rc::Rc, task::Context, task::Poll};

use ntex_service::{Service, ServiceCtx};
use ntex_util::{future::join, task::LocalWaker};

pub trait SizedRequest {
    fn size(&self) -> u32;

    fn is_publish(&self) -> bool;

    fn is_chunk(&self) -> bool;
}

pub struct InFlightServiceImpl<S> {
    count: Counter,
    service: S,
    publish: Cell<bool>,
}

impl<S> fmt::Debug for InFlightServiceImpl<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<S> InFlightServiceImpl<S> {
    pub fn new(max_cap: u16, max_size: usize, service: S) -> Self { panic!("STUB: not implemented") }
}

impl<S, R> Service<R> for InFlightServiceImpl<S>
where
    S: Service<R>,
    R: SizedRequest + 'static,
{
    type Response = S::Response;
    type Error = S::Error;

    #[inline]
    async fn ready(&self, ctx: ServiceCtx<'_, Self>) -> Result<(), S::Error> { panic!("STUB: not implemented") }

    #[inline]
    async fn call(&self, req: R, ctx: ServiceCtx<'_, Self>) -> Result<S::Response, S::Error> { panic!("STUB: not implemented") }

    ntex_service::forward_poll!(service);
    ntex_service::forward_shutdown!(service);
}

struct Counter(Rc<CounterInner>);

struct CounterInner {
    max_cap: u16,
    cur_cap: Cell<u16>,
    max_size: usize,
    cur_size: Cell<usize>,
    task: LocalWaker,
}

impl Counter {
    fn new(max_cap: u16, max_size: usize) -> Self { panic!("STUB: not implemented") }

    fn get(&self, size: u32) -> CounterGuard { panic!("STUB: not implemented") }

    fn is_available(&self) -> bool { panic!("STUB: not implemented") }

    async fn available(&self) { panic!("STUB: not implemented") }
}

struct CounterGuard(u32, Rc<CounterInner>);

impl CounterGuard {
    fn new(size: u32, inner: Rc<CounterInner>) -> Self { panic!("STUB: not implemented") }
}

impl Unpin for CounterGuard {}

impl Drop for CounterGuard {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

impl CounterInner {
    fn inc(&self, size: u32) { panic!("STUB: not implemented") }

    fn dec(&self, size: u32) { panic!("STUB: not implemented") }

    fn available(&self, cx: &Context<'_>) -> bool { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use std::{future::poll_fn, time::Duration};

    use ntex_service::Pipeline;
    use ntex_util::{future::lazy, task::LocalWaker, time::sleep};

    use super::*;

    struct SleepService(Duration);

    impl Service<()> for SleepService {
        type Response = ();
        type Error = ();

        async fn call(&self, _r: (), _: ServiceCtx<'_, Self>) -> Result<(), ()> {
            sleep(self.0).await;
            Ok::<_, ()>(())
        }
    }

    impl SizedRequest for () {
        fn size(&self) -> u32 {
            12
        }

        fn is_publish(&self) -> bool {
            false
        }

        fn is_chunk(&self) -> bool {
            false
        }
    }

    #[ntex::test]
    async fn test_inflight() {
        let wait_time = Duration::from_millis(50);

        let srv = Pipeline::new(InFlightServiceImpl::new(1, 0, SleepService(wait_time))).bind();
        assert_eq!(lazy(|cx| srv.poll_ready(cx)).await, Poll::Ready(Ok(())));

        let srv2 = srv.clone();
        ntex_util::spawn(async move {
            let _ = srv2.call(()).await;
        });
        ntex_util::time::sleep(Duration::from_millis(25)).await;
        assert_eq!(lazy(|cx| srv.poll_ready(cx)).await, Poll::Pending);

        ntex_util::time::sleep(Duration::from_millis(50)).await;
        assert_eq!(lazy(|cx| srv.poll_ready(cx)).await, Poll::Ready(Ok(())));
        assert!(lazy(|cx| srv.poll_shutdown(cx)).await.is_ready());
    }

    #[ntex::test]
    async fn test_inflight2() {
        let wait_time = Duration::from_millis(50);

        let srv =
            Pipeline::new(InFlightServiceImpl::new(0, 10, SleepService(wait_time))).bind();
        assert_eq!(lazy(|cx| srv.poll_ready(cx)).await, Poll::Ready(Ok(())));

        let srv2 = srv.clone();
        ntex_util::spawn(async move {
            let _ = srv2.call(()).await;
        });
        ntex_util::time::sleep(Duration::from_millis(25)).await;
        assert_eq!(lazy(|cx| srv.poll_ready(cx)).await, Poll::Pending);

        ntex_util::time::sleep(Duration::from_millis(100)).await;
        assert_eq!(lazy(|cx| srv.poll_ready(cx)).await, Poll::Ready(Ok(())));
    }

    struct Srv2 {
        dur: Duration,
        cnt: Cell<bool>,
        waker: LocalWaker,
    }

    impl Service<()> for Srv2 {
        type Response = ();
        type Error = ();

        async fn ready(&self, _: ServiceCtx<'_, Self>) -> Result<(), ()> {
            poll_fn(|cx| {
                if self.cnt.get() {
                    self.waker.register(cx.waker());
                    Poll::Pending
                } else {
                    Poll::Ready(Ok(()))
                }
            })
            .await
        }

        async fn call(&self, _r: (), _: ServiceCtx<'_, Self>) -> Result<(), ()> {
            let fut = sleep(self.dur);
            self.cnt.set(true);
            self.waker.wake();

            fut.await;
            self.cnt.set(false);
            self.waker.wake();
            Ok::<_, ()>(())
        }
    }

    #[ntex::test]
    async fn test_inflight3() {
        let wait_time = Duration::from_millis(50);

        let srv = Pipeline::new(InFlightServiceImpl::new(
            1,
            10,
            Srv2 { dur: wait_time, cnt: Cell::new(false), waker: LocalWaker::new() },
        ))
        .bind();
        assert_eq!(lazy(|cx| srv.poll_ready(cx)).await, Poll::Ready(Ok(())));

        let srv2 = srv.clone();
        ntex_util::spawn(async move {
            let _ = srv2.call(()).await;
        });
        ntex_util::time::sleep(Duration::from_millis(25)).await;
        assert_eq!(lazy(|cx| srv.poll_ready(cx)).await, Poll::Pending);

        let srv2 = srv.clone();
        let (tx, rx) = ntex_util::channel::oneshot::channel();
        ntex_util::spawn(async move {
            let _ = poll_fn(|cx| srv2.poll_ready(cx)).await;
            let _ = tx.send(());
        });
        assert_eq!(poll_fn(|cx| srv.poll_ready(cx)).await, Ok(()));

        let _ = rx.await;
    }

    #[test]
    fn test_debug() {
        struct NoopSvc;
        struct Req;
        impl Service<Req> for NoopSvc {
            type Response = ();
            type Error = ();
            async fn call(&self, _: Req, _: ServiceCtx<'_, Self>) -> Result<(), ()> {
                Ok(())
            }
        }
        impl SizedRequest for Req {
            fn size(&self) -> u32 {
                0
            }
            fn is_publish(&self) -> bool {
                false
            }
            fn is_chunk(&self) -> bool {
                false
            }
        }
        let svc = InFlightServiceImpl::new(16, 0, NoopSvc);
        assert!(format!("{svc:?}").contains("InFlightServiceImpl"));
    }
}
