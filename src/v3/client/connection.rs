#![allow(clippy::let_underscore_future)]
use std::{fmt, marker::PhantomData, rc::Rc};

use ntex_io::IoBoxed;
use ntex_router::{IntoPattern, Router, RouterBuilder};
use ntex_service::{IntoService, Pipeline, Service, boxed, fn_service};
use ntex_util::future::{Either, Ready};
use ntex_util::time::{Millis, Seconds, sleep};

use crate::v3::{ProtocolMessageAck, Publish, codec, shared::MqttShared, sink::MqttSink};
use crate::v3::{Session, default::ControlService};
use crate::{control, error::MqttError, io::Dispatcher};

use super::{control::ProtocolMessage, dispatcher::create_dispatcher};

pub struct Client {
    io: IoBoxed,
    shared: Rc<MqttShared>,
    keepalive: Seconds,
    session_present: bool,
    max_receive: usize,
    max_buffer_size: usize,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl Client {
    
    pub(super) fn new(
        io: IoBoxed,
        shared: Rc<MqttShared>,
        session_present: bool,
        keepalive: Seconds,
        max_receive: usize,
        max_buffer_size: usize,
    ) -> Self { panic!("STUB: not implemented") }
}

impl Client {
    #[inline]
    
    pub fn sink(&self) -> MqttSink { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn session_present(&self) -> bool { panic!("STUB: not implemented") }

    pub fn resource<T, F, U>(self, address: T, service: F) -> ClientRouter<U::Error, U::Error>
    where
        T: IntoPattern,
        F: IntoService<U, Publish>,
        U: Service<Publish, Response = ()> + 'static,
    { panic!("STUB: not implemented") }

    pub async fn start_default(self) { panic!("STUB: not implemented") }

    pub async fn start<F, S, E>(self, service: F) -> Result<(), MqttError<E>>
    where
        E: fmt::Debug + 'static,
        F: IntoService<S, ProtocolMessage> + 'static,
        S: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = E> + 'static,
    { panic!("STUB: not implemented") }

    pub async fn start_with_control<F, S, E, C>(
        self,
        service: F,
        control: C,
    ) -> Result<(), MqttError<C::Error>>
    where
        E: fmt::Debug + 'static,
        F: IntoService<S, ProtocolMessage> + 'static,
        S: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = E> + 'static,
        C: Service<control::Control<E>, Response = Option<codec::Encoded>> + 'static,
    { panic!("STUB: not implemented") }

    pub fn into_inner(self) -> (IoBoxed, codec::Codec) { panic!("STUB: not implemented") }
}

type Handler<E> = boxed::BoxService<Publish, (), E>;

pub struct ClientRouter<Err, PErr> {
    builder: RouterBuilder<usize>,
    handlers: Vec<Pipeline<Handler<PErr>>>,
    io: IoBoxed,
    shared: Rc<MqttShared>,
    keepalive: Seconds,
    max_receive: usize,
    max_buffer_size: usize,
    _t: PhantomData<Err>,
}

impl<Err, PErr> fmt::Debug for ClientRouter<Err, PErr> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<Err, PErr> ClientRouter<Err, PErr>
where
    Err: From<PErr> + fmt::Debug + 'static,
    PErr: 'static,
{
    #[must_use]
    
    pub fn resource<T, F, S>(mut self, address: T, service: F) -> Self
    where
        T: IntoPattern,
        F: IntoService<S, Publish>,
        S: Service<Publish, Response = (), Error = PErr> + 'static,
    { panic!("STUB: not implemented") }

    pub async fn start_default(self) { panic!("STUB: not implemented") }

    pub async fn start<F, S>(self, service: F) -> Result<(), MqttError<Err>>
    where
        F: IntoService<S, ProtocolMessage>,
        S: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = Err> + 'static,
    { panic!("STUB: not implemented") }
}

fn dispatch<Err, PErr>(
    router: Router<usize>,
    handlers: Vec<Pipeline<Handler<PErr>>>,
) -> impl Service<Publish, Response = Either<(), Publish>, Error = Err>
where
    PErr: 'static,
    Err: From<PErr>,
{
    let handlers = Rc::new(handlers);

    fn_service(move |mut req: Publish| {
        if let Some((idx, _info)) = router.recognize(req.topic_mut()) {
            
            let idx = *idx;
            let handlers = handlers.clone();
            Either::Left(async move { call(req, handlers[idx].clone()).await })
        } else {
            Either::Right(Ready::<_, Err>::Ok(Either::Right(req)))
        }
    })
}

async fn call<S, Err, PErr>(req: Publish, srv: Pipeline<S>) -> Result<Either<(), Publish>, Err>
where
    S: Service<Publish, Response = (), Error = PErr>,
    Err: From<PErr>,
{ panic!("STUB: not implemented") }

async fn keepalive(sink: MqttSink, timeout: Seconds) { panic!("STUB: not implemented") }
