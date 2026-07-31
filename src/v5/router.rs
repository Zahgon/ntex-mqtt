use std::{cell::RefCell, fmt, num::NonZeroU16, rc::Rc, task::Context};

use ntex_bytes::ByteString;
use ntex_router::{IntoPattern, Path, RouterBuilder};
use ntex_service::boxed::{self, BoxService, BoxServiceFactory};
use ntex_service::{IntoServiceFactory, Service, ServiceCtx, ServiceFactory};
use ntex_util::HashMap;

use super::{Session, publish::Publish, publish::PublishAck};

type Handler<S, E> = BoxServiceFactory<Session<S>, Publish, PublishAck, E, E>;
type HandlerService<E> = BoxService<Publish, PublishAck, E>;

pub struct Router<S, Err> {
    router: RouterBuilder<usize>,
    handlers: Vec<Handler<S, Err>>,
    default: Handler<S, Err>,
}

impl<S, Err> fmt::Debug for Router<S, Err> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<S, Err> Router<S, Err>
where
    S: 'static,
    Err: 'static,
{
    
    pub fn new<F, U>(default_service: F) -> Self
    where
        F: IntoServiceFactory<U, Publish, Session<S>>,
        U: ServiceFactory<
                Publish,
                Session<S>,
                Response = PublishAck,
                Error = Err,
                InitError = Err,
            > + 'static,
    { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn resource<T, F, U>(mut self, address: T, service: F) -> Self
    where
        T: IntoPattern,
        F: IntoServiceFactory<U, Publish, Session<S>>,
        U: ServiceFactory<Publish, Session<S>, Response = PublishAck, Error = Err> + 'static,
        Err: From<U::InitError>,
    { panic!("STUB: not implemented") }

    pub fn build(self) -> RouterFactory<S, Err> { panic!("STUB: not implemented") }

    #[deprecated]
    pub fn finish(self) -> RouterFactory<S, Err> { panic!("STUB: not implemented") }
}

impl<S, Err> IntoServiceFactory<RouterFactory<S, Err>, Publish, Session<S>> for Router<S, Err>
where
    S: 'static,
    Err: 'static,
{
    fn into_factory(self) -> RouterFactory<S, Err> { panic!("STUB: not implemented") }
}

pub struct RouterFactory<S, Err> {
    router: ntex_router::Router<usize>,
    handlers: Rc<Vec<Handler<S, Err>>>,
    default: Handler<S, Err>,
}

impl<S, Err> fmt::Debug for RouterFactory<S, Err> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<S, Err> ServiceFactory<Publish, Session<S>> for RouterFactory<S, Err>
where
    S: 'static,
    Err: 'static,
{
    type Response = PublishAck;
    type Error = Err;
    type InitError = Err;
    type Service = RouterService<Err>;

    async fn create(&self, session: Session<S>) -> Result<Self::Service, Err> { panic!("STUB: not implemented") }
}

pub struct RouterService<Err> {
    router: ntex_router::Router<usize>,
    default: HandlerService<Err>,
    handlers: Vec<HandlerService<Err>>,
    aliases: RefCell<HashMap<NonZeroU16, (usize, Path<ByteString>)>>,
}

impl<Err> fmt::Debug for RouterService<Err> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<Err: 'static> Service<Publish> for RouterService<Err> {
    type Response = PublishAck;
    type Error = Err;

    #[inline]
    async fn ready(&self, ctx: ServiceCtx<'_, Self>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    #[inline]
    fn poll(&self, cx: &mut Context<'_>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    #[allow(clippy::await_holding_refcell_ref)]
    async fn call(
        &self,
        mut req: Publish,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use ntex_service::fn_factory;
    use ntex_util::future::Ready;

    use super::*;
    use crate::v5::codec::PublishAckReason;

    #[test]
    fn test_debug() {
        let router: Router<(), ()> = Router::new(fn_factory(|| async {
            Ok::<_, ()>(ntex_service::fn_service(|_: Publish| {
                Ready::<PublishAck, ()>::Ok(PublishAck::new(PublishAckReason::Success))
            }))
        }));
        assert!(format!("{router:?}").contains("v5::Router"));
    }
}
