use std::{fmt, marker::PhantomData, rc::Rc};

use ntex_codec::{Decoder, Encoder};
use ntex_io::{Filter, Io, IoBoxed};
use ntex_service::{Middleware, Service, ServiceCtx, ServiceFactory, cfg::SharedCfg};
use ntex_util::time::Seconds;

use crate::error::{DecodeError, DispatcherError, EncodeError};
use crate::{control::Control, io::Dispatcher};

type Request<U> = <U as Decoder>::Item;
type Response<U> = Option<<U as Encoder>::Item>;

pub struct MqttServer<St, E, H, T, M, C, Codec> {
    handshake: H,
    handler: Rc<T>,
    middleware: Rc<M>,
    control: Rc<C>,
    _t: PhantomData<(St, E, Codec)>,
}

impl<St, E, H, T, M, C, Codec> fmt::Debug for MqttServer<St, E, H, T, M, C, Codec> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<St, E, H, T, M, C, Codec> MqttServer<St, E, H, T, M, C, Codec> {
    pub(crate) fn new(handshake: H, service: T, mw: M, control: C) -> Self { panic!("STUB: not implemented") }
}

impl<St, E, H, T, M, C, Codec> MqttServer<St, E, H, T, M, C, Codec>
where
    H: ServiceFactory<IoBoxed, SharedCfg, Response = (IoBoxed, Codec, St, Seconds)>,
{
    async fn create_service(
        &self,
        cfg: SharedCfg,
    ) -> Result<MqttHandler<St, E, H::Service, T, M, C, Codec>, H::InitError> { panic!("STUB: not implemented") }
}

impl<St, E, H, T, M, C, Codec> ServiceFactory<IoBoxed, SharedCfg>
    for MqttServer<St, E, H, T, M, C, Codec>
where
    St: Clone + 'static,
    E: 'static,
    H: ServiceFactory<IoBoxed, SharedCfg, Response = (IoBoxed, Codec, St, Seconds)> + 'static,
    T: ServiceFactory<
            Request<Codec>,
            (SharedCfg, St),
            Response = Response<Codec>,
            Error = DispatcherError<E>,
            InitError = H::Error,
        > + 'static,
    M: Middleware<T::Service, (SharedCfg, St)>,
    M::Service: Service<Request<Codec>, Response = Response<Codec>, Error = DispatcherError<E>>
        + 'static,
    C: ServiceFactory<
            Control<E>,
            St,
            Response = Response<Codec>,
            Error = H::Error,
            InitError = H::Error,
        > + 'static,
    Codec: Decoder<Error = DecodeError> + Encoder<Error = EncodeError> + Clone + 'static,
{
    type Response = ();
    type Error = H::Error;
    type InitError = H::InitError;
    type Service = MqttHandler<St, E, H::Service, T, M, C, Codec>;

    async fn create(&self, cfg: SharedCfg) -> Result<Self::Service, Self::InitError> { panic!("STUB: not implemented") }
}

impl<F, St, E, H, T, M, C, Codec> ServiceFactory<Io<F>, SharedCfg>
    for MqttServer<St, E, H, T, M, C, Codec>
where
    F: Filter,
    St: Clone + 'static,
    E: 'static,
    H: ServiceFactory<IoBoxed, SharedCfg, Response = (IoBoxed, Codec, St, Seconds)> + 'static,
    T: ServiceFactory<
            Request<Codec>,
            (SharedCfg, St),
            Response = Response<Codec>,
            Error = DispatcherError<E>,
            InitError = H::Error,
        > + 'static,
    M: Middleware<T::Service, (SharedCfg, St)>,
    M::Service: Service<Request<Codec>, Response = Response<Codec>, Error = DispatcherError<E>>
        + 'static,
    C: ServiceFactory<
            Control<E>,
            St,
            Response = Response<Codec>,
            Error = H::Error,
            InitError = H::Error,
        > + 'static,
    Codec: Decoder<Error = DecodeError> + Encoder<Error = EncodeError> + Clone + 'static,
{
    type Response = ();
    type Error = H::Error;
    type InitError = H::InitError;
    type Service = MqttHandler<St, E, H::Service, T, M, C, Codec>;

    async fn create(&self, cfg: SharedCfg) -> Result<Self::Service, Self::InitError> { panic!("STUB: not implemented") }
}

pub struct MqttHandler<St, E, H, T, M, C, Codec> {
    handshake: H,
    handler: Rc<T>,
    middleware: Rc<M>,
    control: Rc<C>,
    cfg: SharedCfg,
    _t: PhantomData<(St, E, Codec)>,
}

impl<St, E, H, T, M, C, Codec> fmt::Debug for MqttHandler<St, E, H, T, M, C, Codec> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<St, E, H, T, M, C, Codec> Service<IoBoxed> for MqttHandler<St, E, H, T, M, C, Codec>
where
    St: Clone + 'static,
    E: 'static,
    H: Service<IoBoxed, Response = (IoBoxed, Codec, St, Seconds)> + 'static,
    T: ServiceFactory<
            Request<Codec>,
            (SharedCfg, St),
            Response = Response<Codec>,
            Error = DispatcherError<E>,
            InitError = H::Error,
        > + 'static,
    M: Middleware<T::Service, (SharedCfg, St)>,
    M::Service: Service<Request<Codec>, Response = Response<Codec>, Error = DispatcherError<E>>
        + 'static,
    C: ServiceFactory<
            Control<E>,
            St,
            Response = Response<Codec>,
            Error = H::Error,
            InitError = H::Error,
        > + 'static,
    Codec: Decoder<Error = DecodeError> + Encoder<Error = EncodeError> + Clone + 'static,
{
    type Response = ();
    type Error = H::Error;

    ntex_service::forward_ready!(handshake);
    ntex_service::forward_poll!(handshake);
    ntex_service::forward_shutdown!(handshake);

    async fn call(&self, req: IoBoxed, ctx: ServiceCtx<'_, Self>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }
}

impl<F, St, E, H, T, M, C, Codec> Service<Io<F>> for MqttHandler<St, E, H, T, M, C, Codec>
where
    F: Filter,
    St: Clone + 'static,
    E: 'static,
    H: Service<IoBoxed, Response = (IoBoxed, Codec, St, Seconds)> + 'static,
    T: ServiceFactory<
            Request<Codec>,
            (SharedCfg, St),
            Response = Response<Codec>,
            Error = DispatcherError<E>,
            InitError = H::Error,
        > + 'static,
    M: Middleware<T::Service, (SharedCfg, St)>,
    M::Service: Service<Request<Codec>, Response = Response<Codec>, Error = DispatcherError<E>>
        + 'static,
    C: ServiceFactory<
            Control<E>,
            St,
            Response = Response<Codec>,
            Error = H::Error,
            InitError = H::Error,
        > + 'static,
    Codec: Decoder<Error = DecodeError> + Encoder<Error = EncodeError> + Clone + 'static,
{
    type Response = ();
    type Error = H::Error;

    ntex_service::forward_ready!(handshake);
    ntex_service::forward_poll!(handshake);
    ntex_service::forward_shutdown!(handshake);

    #[inline]
    async fn call(&self, io: Io<F>, ctx: ServiceCtx<'_, Self>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }
}
