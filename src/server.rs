use std::{fmt, io, marker, task::Context};

use ntex_codec::{Decoder, Encoder};
use ntex_io::{Filter, Io, IoBoxed};
use ntex_service::{Middleware, Service, ServiceCtx, ServiceFactory, cfg::Cfg, cfg::SharedCfg};
use ntex_util::future::{Either, join, select};
use ntex_util::time::{Deadline, Seconds};

use crate::error::{DecodeError, DispatcherError, EncodeError, HandshakeError, MqttError};
use crate::version::{ProtocolVersion, VersionCodec};
use crate::{MqttServiceConfig, control::Control, service};

type Request<U> = <U as Decoder>::Item;
type Response<U> = Option<<U as Encoder>::Item>;

pub struct MqttServer<V3, V5, Err, InitErr> {
    svc_v3: V3,
    svc_v5: V5,
    _t: marker::PhantomData<(Err, InitErr)>,
}

impl<V3, V5, Err, InitErr> fmt::Debug for MqttServer<V3, V5, Err, InitErr> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<Err, InitErr>
    MqttServer<
        DefaultProtocolServer<Err, InitErr>,
        DefaultProtocolServer<Err, InitErr>,
        Err,
        InitErr,
    >
{
    
    pub fn new() -> Self { panic!("STUB: not implemented") }
}

impl<Err, InitErr> Default
    for MqttServer<
        DefaultProtocolServer<Err, InitErr>,
        DefaultProtocolServer<Err, InitErr>,
        Err,
        InitErr,
    >
{
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl<V3, V5, Err, InitErr> MqttServer<V3, V5, Err, InitErr>
where
    V3: ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (),
            Error = MqttError<Err>,
            InitError = InitErr,
        >,
    V5: ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (),
            Error = MqttError<Err>,
            InitError = InitErr,
        >,
{
    
    pub fn v3<St, E, H, T, M, C, Codec>(
        self,
        service: service::MqttServer<St, E, H, T, M, C, Codec>,
    ) -> MqttServer<
        impl ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (),
            Error = MqttError<Err>,
            InitError = InitErr,
        >,
        V5,
        Err,
        InitErr,
    >
    where
        St: Clone + 'static,
        E: 'static,
        H: ServiceFactory<
                IoBoxed,
                SharedCfg,
                Response = (IoBoxed, Codec, St, Seconds),
                Error = MqttError<Err>,
                InitError = InitErr,
            > + 'static,
        T: ServiceFactory<
                Request<Codec>,
                (SharedCfg, St),
                Response = Response<Codec>,
                Error = DispatcherError<E>,
                InitError = MqttError<Err>,
            > + 'static,
        M: Middleware<T::Service, (SharedCfg, St)>,
        M::Service: Service<Request<Codec>, Response = Response<Codec>, Error = DispatcherError<E>>
            + 'static,
        C: ServiceFactory<
                Control<E>,
                St,
                Response = Response<Codec>,
                Error = MqttError<Err>,
                InitError = MqttError<Err>,
            > + 'static,
        Codec: Encoder<Error = EncodeError> + Decoder<Error = DecodeError> + Clone + 'static,
    {
        MqttServer { svc_v3: service, svc_v5: self.svc_v5, _t: marker::PhantomData }
    }

    pub fn v5<St, E, H, T, M, C, Codec>(
        self,
        service: service::MqttServer<St, E, H, T, M, C, Codec>,
    ) -> MqttServer<
        V3,
        impl ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (),
            Error = MqttError<Err>,
            InitError = InitErr,
        >,
        Err,
        InitErr,
    >
    where
        St: Clone + 'static,
        E: 'static,
        H: ServiceFactory<
                IoBoxed,
                SharedCfg,
                Response = (IoBoxed, Codec, St, Seconds),
                Error = MqttError<Err>,
                InitError = InitErr,
            > + 'static,
        T: ServiceFactory<
                Request<Codec>,
                (SharedCfg, St),
                Response = Response<Codec>,
                Error = DispatcherError<E>,
                InitError = MqttError<Err>,
            > + 'static,
        M: Middleware<T::Service, (SharedCfg, St)>,
        M::Service: Service<Request<Codec>, Response = Response<Codec>, Error = DispatcherError<E>>
            + 'static,
        C: ServiceFactory<
                Control<E>,
                St,
                Response = Response<Codec>,
                Error = MqttError<Err>,
                InitError = MqttError<Err>,
            > + 'static,
        Codec: Encoder<Error = EncodeError> + Decoder<Error = DecodeError> + Clone + 'static,
    {
        MqttServer { svc_v3: self.svc_v3, svc_v5: service, _t: marker::PhantomData }
    }
}

impl<V3, V5, Err, InitErr> MqttServer<V3, V5, Err, InitErr>
where
    V3: ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (),
            Error = MqttError<Err>,
            InitError = InitErr,
        >,
    V5: ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (),
            Error = MqttError<Err>,
            InitError = InitErr,
        >,
{
    async fn create_service(
        &self,
        cfg: SharedCfg,
    ) -> Result<MqttServerImpl<V3::Service, V5::Service, Err>, InitErr> { panic!("STUB: not implemented") }
}

impl<V3, V5, Err, InitErr> ServiceFactory<IoBoxed, SharedCfg>
    for MqttServer<V3, V5, Err, InitErr>
where
    V3: ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (),
            Error = MqttError<Err>,
            InitError = InitErr,
        > + 'static,
    V5: ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (),
            Error = MqttError<Err>,
            InitError = InitErr,
        > + 'static,
    Err: 'static,
    InitErr: 'static,
{
    type Response = ();
    type Error = MqttError<Err>;
    type Service = MqttServerImpl<V3::Service, V5::Service, Err>;
    type InitError = InitErr;

    async fn create(&self, cfg: SharedCfg) -> Result<Self::Service, Self::InitError> { panic!("STUB: not implemented") }
}

impl<F, V3, V5, Err, InitErr> ServiceFactory<Io<F>, SharedCfg>
    for MqttServer<V3, V5, Err, InitErr>
where
    F: Filter,
    V3: ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (),
            Error = MqttError<Err>,
            InitError = InitErr,
        > + 'static,
    V5: ServiceFactory<
            IoBoxed,
            SharedCfg,
            Response = (),
            Error = MqttError<Err>,
            InitError = InitErr,
        > + 'static,
    Err: 'static,
    InitErr: 'static,
{
    type Response = ();
    type Error = MqttError<Err>;
    type Service = MqttServerImpl<V3::Service, V5::Service, Err>;
    type InitError = InitErr;

    async fn create(&self, cfg: SharedCfg) -> Result<Self::Service, Self::InitError> { panic!("STUB: not implemented") }
}

pub struct MqttServerImpl<V3, V5, Err> {
    handlers: (V3, V5),
    cfg: Cfg<MqttServiceConfig>,
    _t: marker::PhantomData<Err>,
}

impl<V3, V5, Err> fmt::Debug for MqttServerImpl<V3, V5, Err> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<V3, V5, Err> Service<IoBoxed> for MqttServerImpl<V3, V5, Err>
where
    V3: Service<IoBoxed, Response = (), Error = MqttError<Err>>,
    V5: Service<IoBoxed, Response = (), Error = MqttError<Err>>,
{
    type Response = ();
    type Error = MqttError<Err>;

    #[inline]
    async fn ready(&self, ctx: ServiceCtx<'_, Self>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    #[inline]
    fn poll(&self, cx: &mut Context<'_>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    #[inline]
    async fn shutdown(&self) { panic!("STUB: not implemented") }

    #[inline]
    async fn call(
        &self,
        io: IoBoxed,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> { panic!("STUB: not implemented") }
}

impl<F, V3, V5, Err> Service<Io<F>> for MqttServerImpl<V3, V5, Err>
where
    F: Filter,
    V3: Service<IoBoxed, Response = (), Error = MqttError<Err>>,
    V5: Service<IoBoxed, Response = (), Error = MqttError<Err>>,
{
    type Response = ();
    type Error = MqttError<Err>;

    #[inline]
    async fn ready(&self, ctx: ServiceCtx<'_, Self>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    #[inline]
    fn poll(&self, cx: &mut Context<'_>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    #[inline]
    async fn shutdown(&self) { panic!("STUB: not implemented") }

    #[inline]
    async fn call(
        &self,
        io: Io<F>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> { panic!("STUB: not implemented") }
}

pub struct DefaultProtocolServer<Err, InitErr> {
    ver: ProtocolVersion,
    _t: marker::PhantomData<(Err, InitErr)>,
}

impl<Err, InitErr> fmt::Debug for DefaultProtocolServer<Err, InitErr> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<Err, InitErr> DefaultProtocolServer<Err, InitErr> {
    fn new(ver: ProtocolVersion) -> Self { panic!("STUB: not implemented") }
}

impl<Err, InitErr> ServiceFactory<IoBoxed, SharedCfg> for DefaultProtocolServer<Err, InitErr> {
    type Response = ();
    type Error = MqttError<Err>;
    type Service = DefaultProtocolServer<Err, InitErr>;
    type InitError = InitErr;

    async fn create(&self, _: SharedCfg) -> Result<Self::Service, Self::InitError> { panic!("STUB: not implemented") }
}

impl<Err, InitErr> Service<IoBoxed> for DefaultProtocolServer<Err, InitErr> {
    type Response = ();
    type Error = MqttError<Err>;

    async fn call(
        &self,
        _: IoBoxed,
        _: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        
        let server = <MqttServer<_, _, (), ()>>::default();
        assert!(format!("{server:?}").contains("MqttServer"));
    }
}
