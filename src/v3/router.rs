use std::{fmt, rc::Rc, task::Context};

use ntex_router::{IntoPattern, RouterBuilder};
use ntex_service::boxed::{self, BoxService, BoxServiceFactory};
use ntex_service::{IntoServiceFactory, Service, ServiceCtx, ServiceFactory};

use super::{Session, publish::Publish};

type Handler<S, E> = BoxServiceFactory<Session<S>, Publish, (), E, E>;
type HandlerService<E> = BoxService<Publish, (), E>;

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
        U: ServiceFactory<Publish, Session<S>, Response = (), Error = Err, InitError = Err>
            + 'static,
    { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn resource<T, F, U>(mut self, address: T, service: F) -> Self
    where
        T: IntoPattern,
        F: IntoServiceFactory<U, Publish, Session<S>>,
        U: ServiceFactory<Publish, Session<S>, Response = (), Error = Err> + 'static,
        Err: From<U::InitError>,
    { panic!("STUB: not implemented") }
}

impl<S, Err> IntoServiceFactory<RouterFactory<S, Err>, Publish, Session<S>> for Router<S, Err>
where
    S: 'static,
    Err: 'static,
{
    fn into_factory(self) -> RouterFactory<S, Err> { panic!("STUB: not implemented") }
}

pub struct RouterFactory<S, Err> {
    router: Rc<ntex_router::Router<usize>>,
    handlers: Vec<Handler<S, Err>>,
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
    type Response = ();
    type Error = Err;
    type InitError = Err;
    type Service = RouterService<Err>;

    async fn create(&self, session: Session<S>) -> Result<Self::Service, Self::Error> { panic!("STUB: not implemented") }
}

pub struct RouterService<Err> {
    router: Rc<ntex_router::Router<usize>>,
    handlers: Vec<HandlerService<Err>>,
    default: HandlerService<Err>,
}

impl<Err> fmt::Debug for RouterService<Err> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<Err> Service<Publish> for RouterService<Err> {
    type Response = ();
    type Error = Err;

    #[inline]
    async fn ready(&self, ctx: ServiceCtx<'_, Self>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    #[inline]
    fn poll(&self, cx: &mut Context<'_>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    #[inline]
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

    #[test]
    fn test_debug() {
        let router: Router<(), ()> = Router::new(fn_factory(|| async {
            Ok::<_, ()>(ntex_service::fn_service(|_: Publish| Ready::<_, ()>::Ok(())))
        }));
        assert!(format!("{router:?}").contains("v3::Router"));
    }
}
