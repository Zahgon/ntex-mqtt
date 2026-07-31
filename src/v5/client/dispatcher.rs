use std::cell::RefCell;
use std::{marker::PhantomData, num::NonZero, num::NonZeroU16, rc::Rc, task::Context};

use ntex_bytes::ByteString;
use ntex_service::{Pipeline, Service, ServiceCtx, cfg::Cfg};
use ntex_util::{HashMap, HashSet, future::Either, future::join};

use crate::error::{DispatcherError, PayloadError, ProtocolError, SpecViolation};
use crate::payload::{Payload, PayloadStatus};
use crate::v5::codec::{Decoded, DisconnectReasonCode, Encoded, Packet};
use crate::v5::shared::{Ack, MqttShared};
use crate::v5::{codec, control::Pkt, publish::Publish, publish::PublishAck};
use crate::{MqttServiceConfig, types::packet_type};

use super::control::{ProtocolMessage, ProtocolMessageAck};

pub(super) fn create_dispatcher<T, C, E>(
    sink: Rc<MqttShared>,
    publish: T,
    control: C,
    max_receive: usize,
    max_topic_alias: u16,
    cfg: Cfg<MqttServiceConfig>,
) -> impl Service<Decoded, Response = Option<Encoded>, Error = DispatcherError<E>>
where
    E: From<T::Error> + 'static,
    T: Service<Publish, Response = Either<Publish, PublishAck>, Error = E> + 'static,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = E> + 'static,
{
    Dispatcher {
        cfg,
        publish,
        max_receive,
        max_topic_alias,
        inner: Rc::new(Inner {
            sink,
            control: Pipeline::new(control.map_err(DispatcherError::Service)),
            info: RefCell::new(PublishInfo {
                aliases: HashMap::default(),
                inflight: HashSet::default(),
            }),
        }),
        _t: PhantomData,
    }
}

pub(crate) struct Dispatcher<T, C, E> {
    publish: T,
    inner: Rc<Inner<C>>,
    max_receive: usize,
    max_topic_alias: u16,
    cfg: Cfg<MqttServiceConfig>,
    _t: PhantomData<E>,
}

struct Inner<C> {
    control: Pipeline<C>,
    sink: Rc<MqttShared>,
    info: RefCell<PublishInfo>,
}

struct PublishInfo {
    inflight: HashSet<NonZeroU16>,
    aliases: HashMap<NonZeroU16, ByteString>,
}

impl<T, C, E> Service<Decoded> for Dispatcher<T, C, E>
where
    E: 'static,
    T: Service<Publish, Response = Either<Publish, PublishAck>, Error = E> + 'static,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>
        + 'static,
{
    type Response = Option<Encoded>;
    type Error = DispatcherError<E>;

    #[inline]
    async fn ready(&self, ctx: ServiceCtx<'_, Self>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    #[inline]
    fn poll(&self, cx: &mut Context<'_>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    async fn shutdown(&self) { panic!("STUB: not implemented") }

    #[allow(clippy::too_many_lines, clippy::await_holding_refcell_ref)]
    async fn call(
        &self,
        request: Decoded,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> { panic!("STUB: not implemented") }
}

async fn publish_fn<'f, T, C, E>(
    svc: &'f T,
    pkt: Publish,
    packet_id: u16,
    packet_size: u32,
    inner: &'f Inner<C>,
    ctx: ServiceCtx<'f, Dispatcher<T, C, E>>,
) -> Result<Option<Encoded>, DispatcherError<E>>
where
    T: Service<Publish, Response = Either<Publish, PublishAck>, Error = E>,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>,
{ panic!("STUB: not implemented") }

impl<C> Inner<C> {
    async fn control<E>(
        &self,
        pkt: ProtocolMessage,
    ) -> Result<Option<Encoded>, DispatcherError<E>>
    where
        C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>,
    { panic!("STUB: not implemented") }

    async fn control_pkt<E>(
        &self,
        pkt: ProtocolMessage,
        packet_id: u16,
    ) -> Result<Option<Encoded>, DispatcherError<E>>
    where
        C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>,
    { panic!("STUB: not implemented") }
}
