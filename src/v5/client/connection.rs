use std::{cell::RefCell, fmt, marker, num::NonZeroU16, rc::Rc};

use ntex_bytes::ByteString;
use ntex_io::IoBoxed;
use ntex_router::{IntoPattern, Path, Router, RouterBuilder};
use ntex_service::{IntoService, Pipeline, Service, boxed, cfg::Cfg, fn_service};
use ntex_util::time::{Millis, Seconds, sleep};
use ntex_util::{HashMap, future::Either, future::Ready};

use crate::v5::default::ControlService;
use crate::v5::publish::{Publish, PublishAck};
use crate::v5::{ProtocolMessageAck, Session, codec, shared::MqttShared, sink::MqttSink};
use crate::{MqttServiceConfig, control, error::MqttError, io::Dispatcher};

use super::{control::ProtocolMessage, dispatcher::create_dispatcher};

pub struct Client {
    io: IoBoxed,
    shared: Rc<MqttShared>,
    keepalive: Seconds,
    max_receive: usize,
    cfg: Cfg<MqttServiceConfig>,
    pkt: Box<codec::ConnectAck>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl Client {
    
    pub(super) fn new(
        io: IoBoxed,
        shared: Rc<MqttShared>,
        pkt: Box<codec::ConnectAck>,
        max_receive: u16,
        keepalive: Seconds,
        cfg: Cfg<MqttServiceConfig>,
    ) -> Self { panic!("STUB: not implemented") }
}

impl Client {
    #[inline]
    
    pub fn sink(&self) -> MqttSink { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn session_present(&self) -> bool { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet(&self) -> &codec::ConnectAck { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet_mut(&mut self) -> &mut codec::ConnectAck { panic!("STUB: not implemented") }

    pub fn resource<T, F, U, E>(self, address: T, service: F) -> ClientRouter<E, U::Error>
    where
        T: IntoPattern,
        F: IntoService<U, Publish>,
        U: Service<Publish, Response = PublishAck> + 'static,
        E: From<U::Error>,
        PublishAck: TryFrom<U::Error, Error = E>,
    { panic!("STUB: not implemented") }

    pub async fn start_default(self) { panic!("STUB: not implemented") }

    pub async fn start<F, S, E>(self, service: F) -> Result<(), MqttError<E>>
    where
        E: fmt::Debug + 'static,
        F: IntoService<S, ProtocolMessage> + 'static,
        S: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = E> + 'static,
    { panic!("STUB: not implemented") }

    pub async fn start_with_control<F, S, C, E>(
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

type Handler<E> = boxed::BoxService<Publish, PublishAck, E>;

pub struct ClientRouter<Err, PErr> {
    io: IoBoxed,
    builder: RouterBuilder<usize>,
    handlers: Vec<Pipeline<Handler<PErr>>>,
    shared: Rc<MqttShared>,
    keepalive: Seconds,
    max_receive: usize,
    cfg: Cfg<MqttServiceConfig>,
    _t: marker::PhantomData<Err>,
}

impl<Err, PErr> fmt::Debug for ClientRouter<Err, PErr> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<Err, PErr> ClientRouter<Err, PErr>
where
    Err: From<PErr> + fmt::Debug + 'static,
    PublishAck: TryFrom<PErr, Error = Err>,
    PErr: fmt::Debug + 'static,
{
    #[must_use]
    
    pub fn resource<T, F, S>(mut self, address: T, service: F) -> Self
    where
        T: IntoPattern,
        F: IntoService<S, Publish>,
        S: Service<Publish, Response = PublishAck, Error = PErr> + 'static,
    { panic!("STUB: not implemented") }

    pub async fn start_default(self) { panic!("STUB: not implemented") }

    pub async fn start<F, S>(self, service: F) -> Result<(), MqttError<Err>>
    where
        F: IntoService<S, ProtocolMessage>,
        S: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = Err> + 'static,
    { panic!("STUB: not implemented") }

    pub fn into_inner(self) -> (IoBoxed, codec::Codec) { panic!("STUB: not implemented") }
}

fn dispatch<Err, PErr>(
    router: Router<usize>,
    handlers: Vec<Pipeline<Handler<PErr>>>,
) -> impl Service<Publish, Response = Either<Publish, PublishAck>, Error = Err>
where
    PErr: 'static,
    PublishAck: TryFrom<PErr, Error = Err>,
{
    
    let aliases: RefCell<HashMap<NonZeroU16, (usize, Path<ByteString>)>> =
        RefCell::new(HashMap::default());
    let handlers = Rc::new(handlers);

    fn_service(move |mut req: Publish| {
        let idx = if !req.publish_topic().is_empty() {
            if let Some((idx, _info)) = router.recognize(req.topic_mut()) {
                
                if let Some(alias) = req.packet().properties.topic_alias {
                    aliases.borrow_mut().insert(alias, (*idx, req.topic().clone()));
                }
                *idx
            } else {
                return Either::Right(Ready::<_, Err>::Ok(Either::Left(req)));
            }
        }
        
        else if let Some(ref alias) = req.packet().properties.topic_alias {
            let aliases = aliases.borrow();
            if let Some(item) = aliases.get(alias) {
                *req.topic_mut() = item.1.clone();
                item.0
            } else {
                log::error!("Unknown topic alias: {alias:?}");
                return Either::Right(Ready::<_, Err>::Ok(Either::Left(req)));
            }
        } else {
            return Either::Right(Ready::<_, Err>::Ok(Either::Left(req)));
        };

        let handlers = handlers.clone();
        Either::Left(async move { call(req, handlers[idx].clone()).await })
    })
}

async fn call<S, Err>(
    req: Publish,
    srv: Pipeline<S>,
) -> Result<Either<Publish, PublishAck>, Err>
where
    S: Service<Publish, Response = PublishAck>,
    PublishAck: TryFrom<S::Error, Error = Err>,
{ panic!("STUB: not implemented") }

async fn keepalive(sink: MqttSink, timeout: Seconds) { panic!("STUB: not implemented") }
