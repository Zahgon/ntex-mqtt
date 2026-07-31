
use std::task::{Context, Poll, ready};
use std::{cell::Cell, cell::RefCell, collections::VecDeque, future::Future, pin::Pin, rc::Rc};

use ntex_codec::{Decoder, Encoder};
use ntex_io::{Decoded, IoBoxed, IoRef, IoStatusUpdate, RecvError};
use ntex_service::{Pipeline, PipelineBinding, PipelineCall, Service};
use ntex_util::channel::condition::Condition;
use ntex_util::{future::Either, future::select, spawn, task::LocalWaker, time::Seconds};

use crate::control::Control;
use crate::error::{DecodeError, DispatcherError, EncodeError, ProtocolError};

type Request<U> = <U as Decoder>::Item;
type Response<U> = <U as Encoder>::Item;
type Queue<T, E> = RefCell<VecDeque<ServiceResult<Result<T, E>>>>;

pin_project_lite::pin_project! {
    
    pub(crate) struct Dispatcher<P, C, U, E>
    where
        P: Service<Request<U>, Response = Option<Response<U>>, Error = DispatcherError<E>>,
        P: 'static,
        C: Service<Control<E>, Response = Option<Response<U>>>,
        C: 'static,
        U: Encoder,
        U: Decoder,
        U: 'static,
        E: 'static
    {
        inner: DispatcherInner<P, C, U, E>
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    struct Flags: u8  {
        const READY_ERR     = 0b0000_0001;
        const IO_ERR        = 0b0000_0010;
        const KA_ENABLED    = 0b0000_0100;
        const KA_TIMEOUT    = 0b0000_1000;
        const READ_TIMEOUT  = 0b0001_0000;
        const READY         = 0b0010_0000;
        const READY_TASK    = 0b0100_0000;
    }
}

struct DispatcherInner<P, C, U, E>
where
    P: Service<Request<U>>,
    C: Service<Control<E>>,
    U: Encoder + Decoder + 'static,
    E: 'static,
{
    io: IoBoxed,
    flags: Flags,
    codec: U,
    service: PipelineBinding<P, Request<U>>,
    control: PipelineBinding<C, Control<E>>,
    st: IoDispatcherState<C, E>,
    state: Rc<DispatcherState<P, U>>,
    stopping: Condition,
    read_remains: u32,
    read_remains_prev: u32,
    read_max_timeout: Seconds,
    keepalive_timeout: Seconds,
}

struct DispatcherState<P, U>
where
    P: Service<Request<U>>,
    U: Encoder + Decoder + 'static,
{
    error: Cell<Option<IoDispatcherError<P::Error>>>,
    base: Cell<usize>,
    queue: Queue<P::Response, P::Error>,
    waker: LocalWaker,
    response: Cell<Option<PipelineCall<P, Request<U>>>>,
    response_idx: Cell<usize>,
}

enum ServiceResult<T> {
    Pending,
    Ready(T),
}

impl<T> ServiceResult<T> {
    fn take(&mut self) -> Option<T> { panic!("STUB: not implemented") }
}

#[derive(Debug)]
enum IoDispatcherState<C: Service<Control<E>>, E: 'static> {
    Processing,
    Backpressure,
    Stop(Option<PipelineCall<C, Control<E>>>),
    Shutdown(Option<Result<(), C::Error>>),
    ShutdownIo(Option<Result<(), C::Error>>),
}

pub(crate) enum IoDispatcherError<S> {
    Encoder(EncodeError),
    Service(S),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PollService {
    Continue,
    Ready,
}

impl<P, C, U, E> Dispatcher<P, C, U, E>
where
    P: Service<Request<U>, Response = Option<Response<U>>, Error = DispatcherError<E>>
        + 'static,
    C: Service<Control<E>, Response = Option<Response<U>>> + 'static,
    U: Decoder<Error = DecodeError> + Encoder<Error = EncodeError> + Clone + 'static,
    <U as Encoder>::Item: 'static,
    E: 'static,
{
    
    pub(crate) fn new(io: IoBoxed, codec: U, service: P, control: C) -> Self { panic!("STUB: not implemented") }

    pub(crate) fn keepalive_timeout(mut self, timeout: Seconds) -> Self { panic!("STUB: not implemented") }
}

impl<P, U> DispatcherState<P, U>
where
    P: Service<Request<U>, Response = Option<Response<U>>> + 'static,
    U: Encoder<Error = EncodeError> + Decoder<Error = DecodeError>,
    <U as Encoder>::Item: 'static,
{
    fn handle_result(
        &self,
        item: Result<P::Response, P::Error>,
        response_idx: usize,
        io: &IoRef,
        codec: &U,
    ) -> bool { panic!("STUB: not implemented") }
}

impl<P, C, U, E> Future for Dispatcher<P, C, U, E>
where
    P: Service<Request<U>, Response = Option<Response<U>>, Error = DispatcherError<E>>
        + 'static,
    C: Service<Control<E>, Response = Option<Response<U>>> + 'static,
    U: Decoder<Error = DecodeError> + Encoder<Error = EncodeError> + Clone + 'static,
    <U as Encoder>::Item: 'static,
    E: 'static,
{
    type Output = Result<(), C::Error>;

    #[allow(clippy::too_many_lines)]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> { panic!("STUB: not implemented") }
}

impl<P, C, U, E> DispatcherInner<P, C, U, E>
where
    P: Service<Request<U>, Response = Option<Response<U>>, Error = DispatcherError<E>>
        + 'static,
    C: Service<Control<E>, Response = Option<Response<U>>> + 'static,
    U: Decoder<Error = DecodeError> + Encoder<Error = EncodeError> + Clone + 'static,
    <U as Encoder>::Item: 'static,
    E: 'static,
{
    fn stop(&mut self, fut: PipelineCall<C, Control<E>>) { panic!("STUB: not implemented") }

    fn call_service(&mut self, cx: &mut Context<'_>, item: Request<U>) { panic!("STUB: not implemented") }

    fn poll_service(&mut self, cx: &mut Context<'_>) -> Poll<PollService> { panic!("STUB: not implemented") }

    fn update_timer(&mut self, decoded: &Decoded<<U as Decoder>::Item>) { panic!("STUB: not implemented") }

    fn handle_timeout(&mut self) -> Result<(), ProtocolError> { panic!("STUB: not implemented") }
}

#[cfg(test)]
#[allow(clippy::items_after_statements)]
mod tests {
    use std::cell::Cell;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use ntex_bytes::{BytePages, Bytes, BytesMut};
    use ntex_io::{self as nio, IoConfig, testing::IoTest as Io};
    use ntex_service::{IntoService, ServiceCtx, cfg::SharedCfg, fn_service};
    use ntex_util::channel::{condition::Condition, oneshot};
    use ntex_util::time::{Millis, sleep};
    use rand::RngExt;

    use super::*;
    use crate::{control::Reason, error::DecodeError, error::EncodeError};

    #[derive(Debug, Copy, Clone)]
    struct BytesCodec;

    impl Encoder for BytesCodec {
        type Item = Bytes;
        type Error = EncodeError;

        #[inline]
        fn encodev(&self, item: Bytes, dst: &mut BytePages) -> Result<(), Self::Error> {
            dst.append(item);
            Ok(())
        }
    }

    impl Decoder for BytesCodec {
        type Item = Bytes;
        type Error = DecodeError;

        fn decode(&self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
            if src.is_empty() {
                Ok(None)
            } else {
                Ok(Some(src.split_to(src.len())))
            }
        }
    }

    impl<P, C, U, E> Dispatcher<P, C, U, E>
    where
        P: Service<Request<U>, Response = Option<Response<U>>, Error = DispatcherError<E>>
            + 'static,
        C: Service<Control<E>, Response = Option<Response<U>>> + 'static,
        U: Decoder<Error = DecodeError> + Encoder<Error = EncodeError> + Clone + 'static,
        E: 'static,
    {
        
        pub(crate) fn new_debug<F: IntoService<P, Request<U>>>(
            io: nio::Io,
            codec: U,
            service: F,
            control: C,
        ) -> (Self, nio::IoRef) {
            let keepalive_timeout = io.cfg().keepalive_timeout();
            let rio = io.get_ref();

            let state = Rc::new(DispatcherState {
                error: Cell::new(None),
                base: Cell::new(0),
                waker: LocalWaker::default(),
                queue: RefCell::new(VecDeque::new()),
                response: Cell::new(None),
                response_idx: Cell::new(0),
            });

            (
                Dispatcher {
                    inner: DispatcherInner {
                        codec,
                        state,
                        keepalive_timeout,
                        stopping: Condition::new(),
                        service: Pipeline::new(service.into_service()).bind(),
                        control: Pipeline::new(control).bind(),
                        io: IoBoxed::from(io),
                        st: IoDispatcherState::Processing,
                        flags: if keepalive_timeout.is_zero() {
                            Flags::empty()
                        } else {
                            Flags::KA_ENABLED
                        },
                        read_remains: 0,
                        read_remains_prev: 0,
                        read_max_timeout: Seconds::ZERO,
                    },
                },
                rio,
            )
        }
    }

    #[ntex::test]
    async fn test_basic() {
        let (client, server) = Io::create();
        client.remote_buffer_cap(1024);
        client.write("GET /test HTTP/1\r\n\r\n");

        let (disp, _) = Dispatcher::new_debug(
            nio::Io::new(server, SharedCfg::new("DBG")),
            BytesCodec,
            fn_service(async move |msg: Bytes| {
                sleep(Millis(50)).await;
                Ok::<_, DispatcherError<()>>(Some(msg))
            }),
            fn_service(async move |_: Control<()>| Ok::<_, ()>(None)),
        );
        ntex_util::spawn(async move {
            let _ = disp.await;
        });
        sleep(Millis(25)).await;
        client.write("GET /test HTTP/1\r\n\r\n");

        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"GET /test HTTP/1\r\n\r\n"));

        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"GET /test HTTP/1\r\n\r\n"));

        client.close().await;
        assert!(client.is_server_dropped());
    }

    #[ntex::test]
    async fn test_drop_connection() {
        let (client, server) = Io::create();
        client.remote_buffer_cap(1024);
        client.write("test");

        #[derive(Clone)]
        struct OnDrop(Rc<Cell<bool>>);
        impl Drop for OnDrop {
            fn drop(&mut self) {
                if Rc::strong_count(&self.0) == 2 {
                    self.0.set(true);
                }
            }
        }
        let ops = Rc::new(Cell::new(false));
        let on_drop = OnDrop(ops.clone());

        let (disp, _) = Dispatcher::new_debug(
            nio::Io::new(server, SharedCfg::new("DBG")),
            BytesCodec,
            fn_service(async move |msg: Bytes| {
                let _on_drop = on_drop.clone();
                if msg == "test" {
                    sleep(Millis(500)).await;
                }
                Ok::<_, DispatcherError<()>>(Some(msg))
            }),
            fn_service(async move |_: Control<()>| Ok::<_, ()>(None)),
        );
        ntex_util::spawn(async move {
            let _ = disp.await;
        });
        sleep(Millis(25)).await;
        client.write("pl1");
        client.close().await;
        assert!(client.is_server_dropped());
        
        assert!(ops.get());
    }

    #[ntex::test]
    async fn test_ordering() {
        let (client, server) = Io::create();
        client.remote_buffer_cap(1024);
        client.write("test");

        let condition = Condition::new();
        let waiter = condition.wait();

        let (disp, _) = Dispatcher::new_debug(
            nio::Io::new(server, SharedCfg::new("DBG")),
            BytesCodec,
            fn_service(async move |msg: Bytes| {
                waiter.clone().await;
                Ok::<_, DispatcherError<()>>(Some(msg))
            }),
            fn_service(async move |_: Control<()>| Ok::<_, ()>(None)),
        );
        ntex_util::spawn(async move {
            let _ = disp.await;
        });
        sleep(Millis(50)).await;

        client.write("test");
        sleep(Millis(50)).await;
        client.write("test");
        sleep(Millis(50)).await;
        condition.notify();

        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"testtesttest"));

        client.close().await;
        assert!(client.is_server_dropped());
    }

    #[ntex::test]
    async fn test_disconnect_ordering() {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum Info {
            Publish,
            PublishDrop,
            Disconnect,
        }

        struct OnDrop(Rc<RefCell<Vec<Info>>>);
        impl Drop for OnDrop {
            fn drop(&mut self) {
                self.0.borrow_mut().push(Info::PublishDrop);
            }
        }

        let condition = Condition::new();
        let waiter = condition.wait();
        let ops = Rc::new(RefCell::new(Vec::new()));
        let ops2 = ops.clone();
        let ops3 = ops.clone();

        let run_server = async || -> Io {
            let (client, server) = Io::create();
            client.remote_buffer_cap(1024);

            let (disp, _) = Dispatcher::new_debug(
                nio::Io::new(server, SharedCfg::new("DBG")),
                BytesCodec,
                fn_service(async move |msg: Bytes| {
                    if msg == b"1" {
                        sleep(Millis(75)).await;
                    } else {
                        ops2.borrow_mut().push(Info::Publish);
                        let on_drop = OnDrop(ops2.clone());
                        waiter.clone().await;
                        drop(on_drop);
                    }
                    Ok::<_, DispatcherError<()>>(Some(msg))
                }),
                fn_service(async move |msg: Control<()>| {
                    if matches!(msg, Control::Stop(Reason::PeerGone(_))) {
                        sleep(Millis(25)).await;
                        ops3.borrow_mut().push(Info::Disconnect);
                    } else {
                        panic!()
                    }
                    Ok::<_, ()>(None)
                }),
            );
            ntex_util::spawn(async move {
                let _ = disp.await;
            });
            sleep(Millis(50)).await;

            client
        };
        let client = run_server.clone()().await;

        client.write("test");
        sleep(Millis(50)).await;
        client.write("test");
        sleep(Millis(50)).await;
        client.close().await;
        assert!(client.is_server_dropped());
        sleep(Millis(150)).await;

        assert_eq!(
            &[
                Info::Publish,
                Info::Publish,
                Info::Disconnect,
                Info::PublishDrop,
                Info::PublishDrop
            ][..],
            &*ops.borrow()
        );

        ops.borrow_mut().clear();
        let client = run_server().await;

        client.write("1");
        sleep(Millis(50)).await;

        client.write("test");
        sleep(Millis(50)).await;
        client.write("test");
        sleep(Millis(50)).await;
        client.close().await;
        assert!(client.is_server_dropped());
        sleep(Millis(150)).await;

        assert_eq!(
            &[
                Info::Publish,
                Info::Publish,
                Info::Disconnect,
                Info::PublishDrop,
                Info::PublishDrop
            ][..],
            &*ops.borrow()
        );
    }

    #[ntex::test]
    async fn test_sink() {
        let (client, server) = Io::create();
        client.remote_buffer_cap(1024);
        client.write("GET /test HTTP/1\r\n\r\n");

        let (disp, io) = Dispatcher::new_debug(
            nio::Io::new(server, SharedCfg::new("DBG")),
            BytesCodec,
            fn_service(async move |msg: Bytes| Ok::<_, DispatcherError<()>>(Some(msg))),
            fn_service(async move |_: Control<()>| Ok::<_, ()>(None)),
        );
        ntex_util::spawn(async move {
            let _ = disp.await;
        });

        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"GET /test HTTP/1\r\n\r\n"));

        assert!(io.encode(Bytes::from_static(b"test"), &BytesCodec).is_ok());
        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"test"));

        io.close();
        sleep(Millis(150)).await;
        assert!(client.is_server_dropped());
    }

    #[ntex::test]
    async fn test_err_in_service() {
        let (client, server) = Io::create();
        client.remote_buffer_cap(0);
        client.write("GET /test HTTP/1\r\n\r\n");

        let (disp, io) = Dispatcher::new_debug(
            nio::Io::new(server, SharedCfg::new("DBG")),
            BytesCodec,
            fn_service(async move |_: Bytes| {
                Err::<Option<Bytes>, _>(DispatcherError::Service(()))
            }),
            fn_service(async move |_: Control<()>| Ok::<_, ()>(None)),
        );
        ntex_util::spawn(async move {
            let _ = disp.await;
        });

        io.encode(Bytes::from_static(b"GET /test HTTP/1\r\n\r\n"), &BytesCodec).unwrap();

        client.remote_buffer_cap(1024);
        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"GET /test HTTP/1\r\n\r\n"));

        sleep(Millis(50)).await;
        assert!(client.is_closed());

        client.close().await;
        assert!(client.is_server_dropped());
    }

    #[ntex::test]
    async fn test_err_in_service_ready() {
        struct Srv(Rc<Cell<usize>>);

        impl Service<Bytes> for Srv {
            type Response = Option<Bytes>;
            type Error = DispatcherError<()>;

            async fn ready(&self, _: ServiceCtx<'_, Self>) -> Result<(), Self::Error> {
                self.0.set(self.0.get() + 1);
                Err(DispatcherError::Service(()))
            }

            async fn call(
                &self,
                _: Bytes,
                _: ServiceCtx<'_, Self>,
            ) -> Result<Option<Bytes>, Self::Error> {
                Ok(None)
            }
        }

        let (client, server) = Io::create();
        client.remote_buffer_cap(0);
        client.write("GET /test HTTP/1\r\n\r\n");

        let counter = Rc::new(Cell::new(0));

        let (disp, io) = Dispatcher::new_debug(
            nio::Io::new(server, SharedCfg::new("DBG")),
            BytesCodec,
            Srv(counter.clone()),
            fn_service(async move |_: Control<()>| Ok::<_, ()>(None)),
        );
        io.encode(Bytes::from_static(b"GET /test HTTP/1\r\n\r\n"), &BytesCodec).unwrap();
        ntex_util::spawn(async move {
            let _ = disp.await;
        });

        client.remote_buffer_cap(1024);
        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"GET /test HTTP/1\r\n\r\n"));

        sleep(Millis(50)).await;
        assert!(client.is_closed());

        client.close().await;
        assert!(client.is_server_dropped());

        assert_eq!(counter.get(), 1);
    }

    #[ntex::test]
    async fn test_write_backpressure() {
        let (client, server) = Io::create();
        
        client.remote_buffer_cap(0);
        client.write("GET /test HTTP/1\r\n\r\n");

        let data = Arc::new(Mutex::new(RefCell::new(Vec::new())));
        let data2 = data.clone();
        let data3 = data.clone();

        let config = SharedCfg::new("DBG").add(
            IoConfig::new().set_read_buf(8 * 1024, 1024, 16).set_write_buf(32 * 1024, 1024, 16),
        );

        let (disp, io) = Dispatcher::new_debug(
            nio::Io::new(server, config),
            BytesCodec,
            fn_service(async move |_: Bytes| {
                data2.lock().unwrap().borrow_mut().push(0);
                let bytes = rand::rng()
                    .sample_iter(&rand::distr::Alphanumeric)
                    .take(65_536)
                    .map(char::from)
                    .collect::<String>();
                Ok::<_, DispatcherError<()>>(Some(Bytes::from(bytes)))
            }),
            fn_service(async move |msg: Control<()>| {
                if let Control::WrBackpressure(st) = msg {
                    if st.enabled() {
                        data3.lock().unwrap().borrow_mut().push(1);
                    } else {
                        data3.lock().unwrap().borrow_mut().push(2);
                    }
                }
                Ok::<_, ()>(None)
            }),
        );

        ntex_util::spawn(async move {
            let _ = disp.await;
        });

        let buf = client.read_any();
        assert_eq!(buf, Bytes::from_static(b""));
        client.write("GET /test HTTP/1\r\n\r\n");
        sleep(Millis(25)).await;

        assert_eq!(client.remote_buffer(|buf| buf.len()), 0);

        assert_eq!(io.with_write_buf(|buf| buf.len()).unwrap(), 65536);

        client.remote_buffer_cap(10240);
        sleep(Millis(50)).await;
        assert_eq!(io.with_write_buf(|buf| buf.len()).unwrap(), 55296);

        client.remote_buffer_cap(45056);
        sleep(Millis(50)).await;
        assert_eq!(io.with_write_buf(|buf| buf.len()).unwrap(), 10240);

        assert_eq!(&data.lock().unwrap().borrow()[..], &[0, 1, 2]);
    }

    #[ntex::test]
    async fn test_shutdown_dispatcher_waker() {
        let (client, server) = Io::create();
        let server = nio::Io::new(server, SharedCfg::new("DBG"));
        client.remote_buffer_cap(1024);

        let flag = Rc::new(Cell::new(true));
        let flag2 = flag.clone();
        let _server_ref = server.get_ref();

        let (disp, _io) = Dispatcher::new_debug(
            server,
            BytesCodec,
            fn_service(async move |item: Bytes| {
                let first = flag2.get();
                flag2.set(false);
                if !first {
                    sleep(Millis(500)).await;
                }
                Ok(Some(item))
            }),
            fn_service(async move |_: Control<()>| Ok::<_, ()>(None)),
        );
        let (tx, rx) = ntex_util::channel::oneshot::channel();
        ntex_util::spawn(async move {
            let _ = disp.await;
            let _ = tx.send(());
        });

        client.write(b"msg1");
        sleep(Millis(25)).await;

        client.write(b"msg2");

        sleep(Millis(150)).await;
        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"msg1"));

        client.close().await;
        let _ = rx.recv().await;
    }

    #[ntex::test]
    async fn test_keepalive() {
        let (client, server) = Io::create();
        client.remote_buffer_cap(1024);

        let data = Arc::new(Mutex::new(RefCell::new(Vec::new())));
        let data2 = data.clone();
        let data3 = data.clone();

        let (disp, _) = Dispatcher::new_debug(
            nio::Io::new(server, SharedCfg::new("DBG")),
            BytesCodec,
            fn_service(async move |msg: Bytes| {
                data2.lock().unwrap().borrow_mut().push(0);
                Ok::<_, DispatcherError<()>>(Some(msg))
            }),
            fn_service(async move |msg: Control<()>| {
                if let Control::Stop(Reason::Protocol(err)) = msg
                    && matches!(err.get_ref(), &ProtocolError::KeepAliveTimeout)
                {
                    data3.lock().unwrap().borrow_mut().push(1);
                }
                Ok::<_, ()>(None)
            }),
        );
        ntex_util::spawn(async move {
            let _ = disp.keepalive_timeout(Seconds(2)).await;
        });

        client.write("1");
        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"1"));
        sleep(Millis(750)).await;

        client.write("2");
        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"2"));

        sleep(Millis(750)).await;
        client.write("3");
        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"3"));

        sleep(Millis(750)).await;
        assert!(!client.is_closed());
        assert_eq!(&data.lock().unwrap().borrow()[..], &[0, 0, 0]);
    }

    #[derive(Debug, Copy, Clone)]
    struct BytesLenCodec(usize);

    impl Encoder for BytesLenCodec {
        type Item = Bytes;
        type Error = EncodeError;

        #[inline]
        fn encode(&self, item: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
            dst.extend_from_slice(&item[..]);
            Ok(())
        }
    }

    impl Decoder for BytesLenCodec {
        type Item = Bytes;
        type Error = DecodeError;

        fn decode(&self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
            if src.len() >= self.0 {
                Ok(Some(src.split_to(self.0)))
            } else {
                Ok(None)
            }
        }
    }

    #[ntex::test]
    async fn test_no_keepalive_err_after_frame_timeout() {
        let (client, server) = Io::create();
        client.remote_buffer_cap(1024);

        let data = Arc::new(Mutex::new(RefCell::new(Vec::new())));
        let data2 = data.clone();
        let data3 = data.clone();

        let config = SharedCfg::new("BDG").add(
            IoConfig::new().set_keepalive_timeout(Seconds(0)).set_frame_read_rate(
                Seconds(1),
                Seconds(2),
                2,
            ),
        );

        let (disp, _) = Dispatcher::new_debug(
            nio::Io::new(server, config),
            BytesLenCodec(2),
            fn_service(async move |msg: Bytes| {
                data2.lock().unwrap().borrow_mut().push(0);
                Ok::<_, DispatcherError<()>>(Some(msg))
            }),
            fn_service(async move |msg: Control<()>| {
                if let Control::Stop(Reason::Protocol(err)) = msg
                    && matches!(err.get_ref(), &ProtocolError::KeepAliveTimeout)
                {
                    data3.lock().unwrap().borrow_mut().push(1);
                }
                Ok::<_, ()>(None)
            }),
        );
        ntex_util::spawn(async move {
            let _ = disp.await;
        });

        client.write("1");
        sleep(Millis(250)).await;
        client.write("2");
        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"12"));
        sleep(Millis(2000)).await;

        assert_eq!(&data.lock().unwrap().borrow()[..], &[0]);
    }

    #[ntex::test]
    async fn test_read_timeout() {
        let (client, server) = Io::create();
        client.remote_buffer_cap(1024);

        let data = Arc::new(Mutex::new(RefCell::new(Vec::new())));
        let data2 = data.clone();
        let data3 = data.clone();

        let config = SharedCfg::new("DBG").add(
            IoConfig::new().set_keepalive_timeout(Seconds::ZERO).set_frame_read_rate(
                Seconds(1),
                Seconds(2),
                2,
            ),
        );

        let (disp, state) = Dispatcher::new_debug(
            nio::Io::new(server, config),
            BytesLenCodec(8),
            fn_service(async move |msg: Bytes| {
                data2.lock().unwrap().borrow_mut().push(0);
                Ok::<_, DispatcherError<()>>(Some(msg))
            }),
            fn_service(async move |msg: Control<()>| {
                if let Control::Stop(Reason::Protocol(err)) = msg
                    && matches!(err.get_ref(), &ProtocolError::ReadTimeout)
                {
                    data3.lock().unwrap().borrow_mut().push(1);
                }
                Ok::<_, ()>(None)
            }),
        );
        ntex_util::spawn(async move {
            let _ = disp.await;
        });

        client.write("12345678");
        let buf = client.read().await.unwrap();
        assert_eq!(buf, Bytes::from_static(b"12345678"));

        client.write("1");
        sleep(Millis(1000)).await;
        assert!(!state.flags().is_stopping());
        client.write("23");
        sleep(Millis(1000)).await;
        assert!(!state.flags().is_stopping());
        client.write("4");
        sleep(Millis(2000)).await;

        assert!(state.flags().is_stopping());
        assert!(client.is_closed());
        assert_eq!(&data.lock().unwrap().borrow()[..], &[0, 1]);
    }

    #[ntex::test]
    async fn cancel_on_stop() {
        #[derive(Clone)]
        struct OnDrop(Arc<AtomicBool>);
        impl Drop for OnDrop {
            fn drop(&mut self) {
                self.0.store(true, Ordering::Relaxed);
            }
        }

        let (client, server) = Io::create();
        client.remote_buffer_cap(1024);

        let data = Arc::new(AtomicBool::new(false));
        let data2 = OnDrop(data.clone());

        let config = SharedCfg::new("DBG").add(
            IoConfig::new().set_keepalive_timeout(Seconds(0)).set_frame_read_rate(
                Seconds(1),
                Seconds(2),
                2,
            ),
        );

        let (disp, _) = Dispatcher::new_debug(
            nio::Io::new(server, config),
            BytesLenCodec(2),
            fn_service(async move |msg: Bytes| {
                let data = data2.clone();
                sleep(Millis(99_9999)).await;
                drop(data);
                Ok::<_, DispatcherError<()>>(Some(msg))
            }),
            fn_service(async move |_: Control<()>| Ok::<_, ()>(None)),
        );
        ntex_util::spawn(async move {
            let _ = disp.await;
        });

        client.write("1");
        client.close().await;
        sleep(Millis(250)).await;

        assert!(&data.load(Ordering::Relaxed));
    }

    #[ntex::test]
    async fn peer_gone_while_service_is_not_ready() {
        #[derive(Clone)]
        struct OnDrop(Arc<AtomicUsize>);
        impl Drop for OnDrop {
            fn drop(&mut self) {
                let cnt = self.0.load(Ordering::Relaxed) + 1;
                self.0.store(cnt, Ordering::Relaxed);
            }
        }

        let data = Arc::new(AtomicUsize::new(0));
        let data2 = OnDrop(data.clone());

        struct Srv(Cell<bool>, OnDrop);

        impl Service<Bytes> for Srv {
            type Response = Option<Bytes>;
            type Error = DispatcherError<()>;

            async fn ready(&self, _: ServiceCtx<'_, Self>) -> Result<(), Self::Error> {
                if self.0.get() {
                    sleep(Millis(999_999)).await;
                }
                Ok(())
            }

            async fn call(
                &self,
                _: Bytes,
                _: ServiceCtx<'_, Self>,
            ) -> Result<Option<Bytes>, Self::Error> {
                let _data = self.1.clone();
                self.0.set(true);
                sleep(Millis(999_999)).await;
                Ok(None)
            }
        }

        let (client, server) = Io::create();
        client.remote_buffer_cap(1024);

        let (disp, _) = Dispatcher::new_debug(
            nio::Io::new(server, SharedCfg::new("DBG")),
            BytesCodec,
            Srv(Cell::new(false), data2),
            fn_service(async move |_: Control<()>| Ok::<_, ()>(None)),
        );
        let (tx, rx) = ntex::channel::oneshot::channel();
        ntex_util::spawn(async move {
            let _ = disp.await;
            let _ = tx.send(());
        });

        client.write("1");
        client.close().await;
        let _ = rx.await;

        let cnt = data.load(Ordering::Relaxed);
        assert_eq!(cnt, 2);
    }

    #[ntex::test]
    async fn service_is_not_ready_and_backpressure() {
        let (ctx, rx) = oneshot::channel();

        struct Srv(Cell<bool>, Cell<Option<oneshot::Receiver<()>>>);

        impl Service<Bytes> for Srv {
            type Response = Option<Bytes>;
            type Error = DispatcherError<()>;

            async fn ready(&self, _: ServiceCtx<'_, Self>) -> Result<(), Self::Error> {
                if self.0.get()
                    && let Some(rx) = self.1.take()
                {
                    let _ = rx.await;
                }
                Ok(())
            }

            async fn call(
                &self,
                msg: Bytes,
                _: ServiceCtx<'_, Self>,
            ) -> Result<Option<Bytes>, Self::Error> {
                self.0.set(true);
                Ok(Some(msg))
            }
        }

        let (client, server) = Io::create();
        client.remote_buffer_cap(0);

        let (disp, _) = Dispatcher::new_debug(
            nio::Io::new(
                server,
                SharedCfg::new("DBG").add(IoConfig::new().set_write_buf(2, 1, 128)),
            ),
            BytesCodec,
            Srv(Cell::new(false), Cell::new(Some(rx))),
            fn_service(async move |_: Control<()>| Ok::<_, ()>(None)),
        );
        let (tx, rx) = ntex::channel::oneshot::channel();
        ntex_util::spawn(async move {
            let _ = disp.await;
            let _ = tx.send(());
        });

        client.write("123456789");
        client.remote_buffer_cap(16);
        let res = client.read().await;
        assert_eq!(res.unwrap(), Bytes::from_static(b"123456789"));
        client.close().await;
        let _ = ctx.send(());
        let _ = rx.await;
    }
}
