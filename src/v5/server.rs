#![allow(clippy::type_complexity)]
use std::{cmp, fmt, marker::PhantomData, num::NonZero, rc::Rc};

use ntex_io::IoBoxed;
use ntex_service::cfg::{Cfg, SharedCfg};
use ntex_service::{Identity, IntoServiceFactory, Service, ServiceCtx, ServiceFactory, Stack};
use ntex_util::time::{Seconds, timeout_checked};

use crate::error::{DispatcherError, HandshakeError, MqttError, ProtocolError};
use crate::{MqttServiceConfig, control, control::Control, service};

use super::codec::{self as mqtt, Decoded, Encoded, Packet};
use super::control::{ProtocolMessage, ProtocolMessageAck};
use super::default::{ControlFactory, DefaultProtocolService, InFlightService};
use super::handshake::{Handshake, HandshakeAck};
use super::publish::{Publish, PublishAck};
use super::shared::{MqttShared, MqttSinkPool};
use super::{MqttSink, Session, ToPublishAck, dispatcher::factory};

pub struct MqttServer<St, E, H, P, C, M = Identity> {
    control: C,
    handshake: H,
    protocol: P,
    middleware: M,
    pub(super) pool: Rc<MqttSinkPool>,
    _t: PhantomData<(St, E)>,
}

impl<St, E, H, P, C, M> fmt::Debug for MqttServer<St, E, H, P, C, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<St, E, H>
    MqttServer<
        St,
        E,
        H,
        DefaultProtocolService<Session<St>, E>,
        ControlFactory<
            control::DefaultControlService<Session<St>, E, Encoded, H::Error>,
            St,
            E,
        >,
        InFlightService,
    >
where
    H: ServiceFactory<Handshake, SharedCfg, Response = HandshakeAck<St>>,
{
    
    pub fn new<F>(handshake: F) -> Self
    where
        F: IntoServiceFactory<H, Handshake, SharedCfg>,
    { panic!("STUB: not implemented") }
}

impl<St, E, H, P, C, M> MqttServer<St, E, H, P, C, M>
where
    St: 'static,
    H: ServiceFactory<Handshake, SharedCfg, Response = HandshakeAck<St>> + 'static,
    P: ServiceFactory<ProtocolMessage, Session<St>, Response = ProtocolMessageAck> + 'static,
    C: ServiceFactory<
            Control<E>,
            Session<St>,
            Response = Option<Encoded>,
            Error = MqttError<H::Error>,
            InitError = MqttError<H::Error>,
        > + 'static,
{
    
    pub fn middleware<U>(self, mw: U) -> MqttServer<St, E, H, P, C, Stack<M, U>> { panic!("STUB: not implemented") }

    pub fn replace_middlewares<U>(self, mw: U) -> MqttServer<St, E, H, P, C, U> { panic!("STUB: not implemented") }

    pub fn protocol<F, Srv>(self, service: F) -> MqttServer<St, E, H, Srv, C, M>
    where
        F: IntoServiceFactory<Srv, ProtocolMessage, Session<St>>,
        Srv: ServiceFactory<ProtocolMessage, Session<St>, Response = ProtocolMessageAck>
            + 'static,
        E: From<Srv::Error>,
        H::Error: From<Srv::InitError>,
    { panic!("STUB: not implemented") }

    pub fn control<F, Srv>(
        self,
        service: F,
    ) -> MqttServer<
        St,
        E,
        H,
        P,
        impl ServiceFactory<
            Control<E>,
            Session<St>,
            Response = Option<Encoded>,
            Error = MqttError<H::Error>,
            InitError = MqttError<H::Error>,
        >,
        M,
    >
    where
        F: IntoServiceFactory<Srv, Control<E>, Session<St>>,
        Srv: ServiceFactory<Control<E>, Session<St>, Response = Option<Encoded>> + 'static,
        H::Error: From<Srv::Error> + From<Srv::InitError>,
    {
        MqttServer {
            handshake: self.handshake,
            protocol: self.protocol,
            control: ControlFactory::new(
                service.into_factory().map_err(H::Error::from).map_init_err(H::Error::from),
            ),
            middleware: self.middleware,
            pool: self.pool,
            _t: PhantomData,
        }
    }

    pub fn publish<F, Srv>(
        self,
        publish: F,
    ) -> service::MqttServer<
        Session<St>,
        E,
        impl ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (IoBoxed, Rc<MqttShared>, Session<St>, Seconds),
            Error = MqttError<H::Error>,
            InitError = H::InitError,
        >,
        impl ServiceFactory<
            Decoded,
            (SharedCfg, Session<St>),
            Response = Option<Encoded>,
            Error = DispatcherError<E>,
            InitError = MqttError<H::Error>,
        >,
        M,
        C,
        Rc<MqttShared>,
    >
    where
        F: IntoServiceFactory<Srv, Publish, Session<St>>,
        H::Error: From<P::InitError> + From<Srv::InitError>,
        E: From<P::Error> + 'static,
        Srv: ServiceFactory<Publish, Session<St>, Response = PublishAck> + 'static,
        Srv::Error: ToPublishAck<Error = E>,
    {
        service::MqttServer::new(
            HandshakeFactory { factory: self.handshake, pool: self.pool, _t: PhantomData },
            factory(publish.into_factory(), self.protocol),
            self.middleware,
            self.control,
        )
    }
}

struct HandshakeFactory<St, H> {
    factory: H,
    pool: Rc<MqttSinkPool>,
    _t: PhantomData<St>,
}

impl<St, H> ServiceFactory<IoBoxed, SharedCfg> for HandshakeFactory<St, H>
where
    H: ServiceFactory<Handshake, SharedCfg, Response = HandshakeAck<St>> + 'static,
{
    type Response = (IoBoxed, Rc<MqttShared>, Session<St>, Seconds);
    type Error = MqttError<H::Error>;

    type Service = HandshakeService<St, H::Service>;
    type InitError = H::InitError;

    async fn create(&self, cfg: SharedCfg) -> Result<Self::Service, Self::InitError> { panic!("STUB: not implemented") }
}

struct HandshakeService<St, H> {
    service: H,
    cfg: Cfg<MqttServiceConfig>,
    pool: Rc<MqttSinkPool>,
    _t: PhantomData<St>,
}

impl<St, H> Service<IoBoxed> for HandshakeService<St, H>
where
    H: Service<Handshake, Response = HandshakeAck<St>> + 'static,
{
    type Response = (IoBoxed, Rc<MqttShared>, Session<St>, Seconds);
    type Error = MqttError<H::Error>;

    ntex_service::forward_ready!(service, MqttError::Service);
    ntex_service::forward_poll!(service, MqttError::Service);
    ntex_service::forward_shutdown!(service);

    async fn call(
        &self,
        io: IoBoxed,
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
        let server = MqttServer::<(), (), _, _, _, _>::new(fn_factory(|| async {
            Ok::<_, ()>(ntex_service::fn_service(|h: Handshake| {
                Ready::<HandshakeAck<()>, ()>::Ok(h.ack(()))
            }))
        }));
        assert!(format!("{server:?}").contains("v5::MqttServer"));
    }
}
