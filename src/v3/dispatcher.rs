use std::cell::RefCell;
use std::{marker::PhantomData, num::NonZeroU16, rc::Rc, task::Context};

use ntex_service::cfg::{Cfg, SharedCfg};
use ntex_service::{Pipeline, PipelineSvc, Service, ServiceCtx, ServiceFactory};
use ntex_util::services::buffer::{BufferService, BufferServiceError};
use ntex_util::{HashSet, future::join, services::inflight::InFlightService};

use crate::error::{
    DecodeError, DispatcherError, MqttError, PayloadError, ProtocolError, SpecViolation,
};
use crate::payload::{Payload, PayloadStatus};
use crate::{MqttServiceConfig, types::QoS, types::packet_type};

use super::codec::{Decoded, Encoded, Packet};
use super::control::{
    ProtocolMessage, ProtocolMessageAck, ProtocolMessageKind, Subscribe, Unsubscribe,
};
use super::{Session, publish::Publish, shared::Ack, shared::MqttShared};

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
    T: ServiceFactory<Publish, Session<St>, Response = ()> + 'static,
    P: ServiceFactory<ProtocolMessage, Session<St>, Response = ProtocolMessageAck> + 'static,
    E: From<T::Error> + From<P::Error> + 'static,
    InitErr: From<T::InitError> + From<P::InitError>,
{
    let factories = Rc::new((publish, control));

    ntex_service::fn_factory_with_config(
        async move |(cfg, session): (SharedCfg, Session<St>)| {
            
            let sink = session.sink().shared();
            let fut = join(factories.0.create(session.clone()), factories.1.create(session));
            let (publish, control) = fut.await;

            let publish = publish.map_err(|e| MqttError::Service(InitErr::from(e)))?;
            let control = Pipeline::new(
                control
                    .map_err(|e| MqttError::Service(InitErr::from(e)))?
                    .map_err(|e| DispatcherError::Service(e.into())),
            );

            let control = Pipeline::new(
                BufferService::new(
                    16,
                    
                    InFlightService::new(1, PipelineSvc::new(control.clone())),
                )
                .map_err(|err| match err {
                    BufferServiceError::Service(e) => e,
                    BufferServiceError::RequestCanceled => {
                        DispatcherError::Protocol(ProtocolError::ReadTimeout)
                    }
                }),
            );

            let cfg: Cfg<MqttServiceConfig> = cfg.get();
            Ok(Dispatcher::new(sink, publish, control, cfg))
        },
    )
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
    inflight: RefCell<HashSet<NonZeroU16>>,
}

impl<T, C, E> Dispatcher<T, C, E>
where
    E: From<T::Error>,
    T: Service<Publish, Response = ()>,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>,
{
    pub(crate) fn new(
        sink: Rc<MqttShared>,
        publish: T,
        control: Pipeline<C>,
        cfg: Cfg<MqttServiceConfig>,
    ) -> Self { panic!("STUB: not implemented") }

    fn tag(&self) -> &'static str { panic!("STUB: not implemented") }
}

impl<T, C, E> Service<Decoded> for Dispatcher<T, C, E>
where
    E: From<T::Error> + 'static,
    T: Service<Publish, Response = ()> + 'static,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>
        + 'static,
{
    type Response = Option<Encoded>;
    type Error = DispatcherError<E>;

    #[inline]
    async fn ready(&self, ctx: ServiceCtx<'_, Self>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    fn poll(&self, cx: &mut Context<'_>) -> Result<(), Self::Error> { panic!("STUB: not implemented") }

    async fn shutdown(&self) { panic!("STUB: not implemented") }

    #[allow(clippy::too_many_lines)]
    async fn call(
        &self,
        req: Decoded,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> { panic!("STUB: not implemented") }
}

async fn publish_fn<'f, T, C, E>(
    svc: &'f T,
    pkt: Publish,
    packet_id: Option<NonZeroU16>,
    inner: &'f Inner<C>,
    ctx: ServiceCtx<'f, Dispatcher<T, C, E>>,
) -> Result<Option<Encoded>, DispatcherError<E>>
where
    E: From<T::Error>,
    T: Service<Publish, Response = ()>,
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
}

#[cfg(test)]
mod tests {
    use std::{future::Future, pin::Pin};

    use ntex_bytes::{ByteString, Bytes};
    use ntex_io::{Io, testing::IoTest};
    use ntex_service::{cfg::SharedCfg, fn_service};
    use ntex_util::{future::lazy, time::Seconds, time::sleep};

    use super::*;
    use crate::{error, v3::codec};

    #[ntex::test]
    async fn test_dup_packet_id() {
        let cfg: SharedCfg = SharedCfg::new("DBG")
            .add(MqttServiceConfig::new().set_max_qos(QoS::AtLeastOnce))
            .into();

        let io = Io::new(IoTest::create().0, cfg.clone());
        let codec = codec::Codec::default();
        let shared = Rc::new(MqttShared::new(io.get_ref(), codec, false, Rc::default()));

        let disp = Pipeline::new(Dispatcher::new(
            shared.clone(),
            fn_service(async |_| {
                sleep(Seconds(10)).await;
                Ok(())
            }),
            Pipeline::new(fn_service(async |msg: ProtocolMessage| {
                Ok::<_, DispatcherError<()>>(msg.ack())
            })),
            cfg.get(),
        ));

        let mut f: Pin<Box<dyn Future<Output = Result<_, _>>>> =
            Box::pin(disp.call(Decoded::Publish(
                codec::Publish {
                    dup: false,
                    retain: false,
                    qos: QoS::AtLeastOnce,
                    topic: ByteString::new(),
                    packet_id: NonZeroU16::new(1),
                    payload_size: 0,
                },
                Bytes::new(),
                999,
            )));
        let _ = lazy(|cx| Pin::new(&mut f).poll(cx)).await;

        let f = Box::pin(disp.call(Decoded::Publish(
            codec::Publish {
                dup: false,
                retain: false,
                qos: QoS::AtLeastOnce,
                topic: ByteString::new(),
                packet_id: NonZeroU16::new(1),
                payload_size: 0,
            },
            Bytes::new(),
            999,
        )));

        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) =
            f.await.err().unwrap()
        else {
            panic!()
        };
        assert_eq!(
            err.inner,
            error::ViolationInner::Spec(error::SpecViolation::PacketId_2_2_1_3_Pub)
        );
    }

    #[ntex::test]
    async fn test_spec_violations() {
        let cfg: SharedCfg = SharedCfg::new("DBG")
            .add(MqttServiceConfig::new().set_max_qos(QoS::AtLeastOnce))
            .into();

        let io = Io::new(IoTest::create().0, cfg.clone());
        let codec = codec::Codec::default();
        let shared = Rc::new(MqttShared::new(io.get_ref(), codec, false, Rc::default()));

        let disp = Pipeline::new(Dispatcher::new(
            shared.clone(),
            fn_service(async |_: Publish| Ok::<_, ()>(())),
            Pipeline::new(fn_service(async |msg: ProtocolMessage| {
                Ok::<_, DispatcherError<()>>(msg.ack())
            })),
            cfg.get(),
        ));

        let err = disp
            .call(Decoded::Packet(
                Packet::PublishAck { packet_id: NonZeroU16::new(100).unwrap() },
                999,
            ))
            .await
            .err()
            .unwrap();
        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) = err else {
            panic!()
        };
        let error::ViolationInner::Common { reason, .. } = err.inner else { panic!() };
        assert_eq!(reason, crate::v5::codec::DisconnectReasonCode::ProtocolError);

        let err = disp
            .call(Decoded::Packet(
                Packet::PublishReceived { packet_id: NonZeroU16::new(100).unwrap() },
                999,
            ))
            .await
            .err()
            .unwrap();
        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) = err else {
            panic!()
        };
        let error::ViolationInner::Common { reason, .. } = err.inner else { panic!() };
        assert_eq!(reason, crate::v5::codec::DisconnectReasonCode::ProtocolError);

        let err = disp
            .call(Decoded::Packet(
                Packet::PublishRelease { packet_id: NonZeroU16::new(100).unwrap() },
                999,
            ))
            .await
            .err()
            .unwrap();
        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) = err else {
            panic!()
        };
        let error::ViolationInner::UnexpectedPacket { packet_type, .. } = err.inner else {
            panic!()
        };
        assert_eq!(packet_type, 98);

        let err = disp
            .call(Decoded::Packet(
                Packet::PublishComplete { packet_id: NonZeroU16::new(100).unwrap() },
                999,
            ))
            .await
            .err()
            .unwrap();
        let DispatcherError::Protocol(ProtocolError::ProtocolViolation(err)) = err else {
            panic!()
        };
        let error::ViolationInner::Common { reason, .. } = err.inner else { panic!() };
        assert_eq!(reason, crate::v5::codec::DisconnectReasonCode::ProtocolError);

        let err = disp
            .call(Decoded::Packet(
                Packet::Subscribe {
                    packet_id: NonZeroU16::new(1).unwrap(),
                    topic_filters: vec![(ByteString::new(), QoS::AtLeastOnce)],
                },
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
                Packet::Unsubscribe {
                    packet_id: NonZeroU16::new(1).unwrap(),
                    topic_filters: vec![ByteString::new()],
                },
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
