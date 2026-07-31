use std::{cell::RefCell, marker::PhantomData, num, rc::Rc, task::Context};

use ntex_bytes::ByteString;
use ntex_service::cfg::{Cfg, SharedCfg};
use ntex_service::{self as service, Pipeline, Service, ServiceCtx, ServiceFactory};
use ntex_util::services::buffer::{BufferService, BufferServiceError};
use ntex_util::services::inflight::InFlightService;
use ntex_util::{HashMap, HashSet, future::join};

use crate::error::{
    DecodeError, DispatcherError, MqttError, PayloadError, ProtocolError, SpecViolation,
};
use crate::payload::{Payload, PayloadStatus};
use crate::{MqttServiceConfig, types::QoS};

use super::codec::{self, Decoded, DisconnectReasonCode, Encoded, Packet};
use super::control::{Pkt, ProtocolMessage, ProtocolMessageAck};
use super::publish::{Publish, PublishAck};
use super::{Session, ToPublishAck, shared::Ack, shared::MqttShared};

pub(super) fn factory<St, T, P, E, InitErr>(
    publish: T,
    control: P,
) -> impl ServiceFactory<
    Decoded,
    (SharedCfg, Session<St>),
    Response = Option<Encoded>,
    Error = DispatcherError<E>,
    InitError = MqttError<InitErr>,
>
where
    St: 'static,
    E: From<P::Error> + 'static,
    T: ServiceFactory<Publish, Session<St>, Response = PublishAck> + 'static,
    T::Error: ToPublishAck<Error = E>,
    P: ServiceFactory<ProtocolMessage, Session<St>, Response = ProtocolMessageAck> + 'static,
    InitErr: From<T::InitError> + From<P::InitError>,
{
    let factories = Rc::new((publish, control));

    service::fn_factory_with_config(async move |(cfg, ses): (SharedCfg, Session<St>)| {
        let cfg: Cfg<MqttServiceConfig> = cfg.get();

        let sink = ses.sink().shared();
        let (publish, control) =
            join(factories.0.create(ses.clone()), factories.1.create(ses)).await;

        let publish = publish.map_err(|e| MqttError::Service(InitErr::from(e)))?;
        let control = control
            .map_err(|e| MqttError::Service(InitErr::from(e)))?
            .map_err(|e| DispatcherError::Service(e.into()));

        let control = Pipeline::new(
            BufferService::new(
                16,
                
                InFlightService::new(1, control),
            )
            .map_err(|err| match err {
                BufferServiceError::Service(e) => e,
                BufferServiceError::RequestCanceled => {
                    DispatcherError::Protocol(ProtocolError::ReadTimeout)
                }
            }),
        );

        Ok(Dispatcher::new(sink, publish, control, cfg))
    })
}

impl crate::inflight::SizedRequest for Decoded {
    fn size(&self) -> u32 { panic!("STUB: not implemented") }

    fn is_publish(&self) -> bool { panic!("STUB: not implemented") }

    fn is_chunk(&self) -> bool { panic!("STUB: not implemented") }
}

pub(crate) struct Dispatcher<T, C, E> {
    publish: T,
    inner: Rc<Inner<C>>,
    cfg: Cfg<MqttServiceConfig>,
    _t: PhantomData<E>,
}

struct Inner<C> {
    control: Pipeline<C>,
    sink: Rc<MqttShared>,
    info: RefCell<PublishInfo>,
}

struct PublishInfo {
    inflight: HashSet<num::NonZeroU16>,
    aliases: HashMap<num::NonZeroU16, ByteString>,
}

impl<T, C, E> Dispatcher<T, C, E>
where
    T: Service<Publish, Response = PublishAck>,
    T::Error: ToPublishAck<Error = E>,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>,
{
    fn new(
        sink: Rc<MqttShared>,
        publish: T,
        control: Pipeline<C>,
        cfg: Cfg<MqttServiceConfig>,
    ) -> Self { panic!("STUB: not implemented") }

    fn tag(&self) -> &'static str { panic!("STUB: not implemented") }
}

impl<T, C, E> Service<Decoded> for Dispatcher<T, C, E>
where
    T: Service<Publish, Response = PublishAck> + 'static,
    T::Error: ToPublishAck<Error = E>,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>
        + 'static,
{
    type Response = Option<Encoded>;
    type Error = DispatcherError<E>;

    async fn ready(&self, ctx: ServiceCtx<'_, Self>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    fn poll(&self, cx: &mut Context<'_>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    async fn shutdown(&self) { panic!("STUB: not implemented") }

    #[allow(clippy::too_many_lines, clippy::await_holding_refcell_ref)]
    async fn call(
        &self,
        request: Decoded,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> { panic!("STUB: not implemented") }
}

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

async fn publish_fn<'f, T, C, E>(
    publish: &T,
    pkt: Publish,
    packet_id: u16,
    inner: &'f Inner<C>,
    ctx: ServiceCtx<'f, Dispatcher<T, C, E>>,
) -> Result<Option<Encoded>, DispatcherError<E>>
where
    T: Service<Publish, Response = PublishAck>,
    T::Error: ToPublishAck<Error = E>,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>,
{ panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use std::num::{NonZeroU16, NonZeroU32};

    use ntex_bytes::{ByteString, Bytes};
    use ntex_io::{Io, testing::IoTest};
    use ntex_service::{cfg::SharedCfg, fn_service};

    use super::*;
    use crate::{error, v5::codec};

    #[derive(Debug)]
    struct TestError;

    impl From<()> for TestError {
        fn from((): ()) -> Self {
            TestError
        }
    }

    impl TryFrom<TestError> for PublishAck {
        type Error = TestError;

        fn try_from(err: TestError) -> Result<Self, Self::Error> {
            Err(err)
        }
    }

    #[ntex::test]
    async fn test_spec_violations() {
        let cfg: SharedCfg = SharedCfg::new("DBG")
            .add(MqttServiceConfig::new().set_max_qos(QoS::AtLeastOnce))
            .into();

        let io = Io::new(IoTest::create().0, cfg.clone());
        let codec = codec::Codec::default();
        codec.set_retain_available(false);
        codec.set_sub_ids_available(false);
        let shared = Rc::new(MqttShared::new(io.get_ref(), codec, Rc::default()));
        shared.set_topic_alias_max(1);

        let disp = Pipeline::new(Dispatcher::new(
            shared.clone(),
            fn_service(async |msg: Publish| Ok::<_, TestError>(msg.ack())),
            Pipeline::new(fn_service(async |msg: ProtocolMessage| {
                Ok::<_, DispatcherError<TestError>>(msg.ack())
            })),
            cfg.get(),
        ));

        let err = disp
            .call(Decoded::Publish(
                codec::Publish {
                    retain: true,
                    qos: QoS::AtLeastOnce,
                    packet_id: NonZeroU16::new(1),
                    ..Default::default()
                },
                Bytes::new(),
                999,
            ))
            .await
            .err()
            .unwrap();

        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) = err else {
            panic!()
        };
        assert_eq!(
            err.inner,
            error::ViolationInner::Spec(error::SpecViolation::Connack_3_2_2_14)
        );

        let mut pkt = codec::Publish::default();
        pkt.properties.topic_alias = NonZeroU16::new(1);

        let err = disp.call(Decoded::Publish(pkt, Bytes::new(), 999)).await.err().unwrap();
        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) = err else {
            panic!()
        };
        assert_eq!(
            err.inner,
            error::ViolationInner::Common {
                reason: DisconnectReasonCode::TopicAliasInvalid,
                message: "Unknown topic alias"
            }
        );

        let mut pkt = codec::Publish {
            packet_id: NonZeroU16::new(1),
            topic: ByteString::from_static("test"),
            ..Default::default()
        };
        pkt.properties.topic_alias = NonZeroU16::new(1);
        let res = disp.call(Decoded::Publish(pkt, Bytes::new(), 999)).await;
        assert!(res.is_ok());

        let mut pkt = codec::Publish {
            packet_id: NonZeroU16::new(2),
            topic: ByteString::from_static("test2"),
            ..Default::default()
        };
        pkt.properties.topic_alias = NonZeroU16::new(2);

        let err = disp.call(Decoded::Publish(pkt, Bytes::new(), 999)).await.err().unwrap();
        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) = err else {
            panic!()
        };
        assert_eq!(
            err.inner,
            error::ViolationInner::Spec(error::SpecViolation::Connack_3_2_2_17)
        );

        let pkt = disp
            .call(Decoded::Packet(
                Packet::PublishRelease(codec::PublishAck2 {
                    packet_id: NonZeroU16::new(100).unwrap(),
                    reason_code: codec::PublishAck2Reason::Success,
                    properties: codec::UserProperties::default(),
                    reason_string: None,
                }),
                999,
            ))
            .await
            .ok()
            .unwrap()
            .unwrap();

        let Encoded::Packet(Packet::PublishComplete(pkt)) = pkt else { panic!() };
        assert_eq!(pkt.reason_code, codec::PublishAck2Reason::PacketIdNotFound);

        let err = disp
            .call(Decoded::Packet(
                Packet::Subscribe(codec::Subscribe {
                    packet_id: NonZeroU16::new(1).unwrap(),
                    id: None,
                    user_properties: codec::UserProperties::default(),
                    topic_filters: vec![(
                        ByteString::new(),
                        codec::SubscriptionOptions::default(),
                    )],
                }),
                999,
            ))
            .await
            .err()
            .unwrap();

        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) = err else {
            panic!()
        };
        assert_eq!(err.inner, error::ViolationInner::Spec(error::SpecViolation::Subs_4_7_1));

        let err = disp
            .call(Decoded::Packet(
                Packet::Subscribe(codec::Subscribe {
                    packet_id: NonZeroU16::new(1).unwrap(),
                    id: NonZeroU32::new(1),
                    user_properties: codec::UserProperties::default(),
                    topic_filters: vec![(
                        ByteString::from_static("test"),
                        codec::SubscriptionOptions::default(),
                    )],
                }),
                999,
            ))
            .await
            .err()
            .unwrap();

        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) = err else {
            panic!()
        };
        assert_eq!(
            err.inner,
            error::ViolationInner::Spec(error::SpecViolation::Connack_3_2_2_3_12)
        );

        let err = disp
            .call(Decoded::Packet(
                Packet::Unsubscribe(codec::Unsubscribe {
                    packet_id: NonZeroU16::new(1).unwrap(),
                    user_properties: codec::UserProperties::default(),
                    topic_filters: vec![ByteString::new()],
                }),
                999,
            ))
            .await
            .err()
            .unwrap();

        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) = err else {
            panic!()
        };
        assert_eq!(err.inner, error::ViolationInner::Spec(error::SpecViolation::Subs_4_7_1));
    }
}
