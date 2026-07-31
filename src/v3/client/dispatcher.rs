use std::cell::{Cell, RefCell};
use std::{marker::PhantomData, num::NonZeroU16, rc::Rc, task::Context};

use ntex_service::{Pipeline, Service, ServiceCtx};
use ntex_util::future::{Either, join};
use ntex_util::{HashSet, services::inflight::InFlightService};

use crate::error::{DispatcherError, PayloadError, ProtocolError, SpecViolation};
use crate::v3::codec::{self, Decoded, Encoded, Packet};
use crate::v3::shared::{Ack, MqttShared};
use crate::v3::{control::ProtocolMessageKind, publish::Publish};
use crate::{payload::Payload, payload::PayloadStatus, payload::PlSender};

use super::control::{ProtocolMessage, ProtocolMessageAck};

pub(super) fn create_dispatcher<T, C, E>(
    sink: Rc<MqttShared>,
    inflight: usize,
    max_buffer_size: usize,
    publish: T,
    control: C,
) -> impl Service<Decoded, Response = Option<Encoded>, Error = DispatcherError<E>>
where
    E: 'static,
    T: Service<Publish, Response = Either<(), Publish>, Error = E> + 'static,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = E> + 'static,
{
    
    InFlightService::new(
        inflight,
        Dispatcher::new(
            sink,
            publish,
            control.map_err(DispatcherError::Service),
            max_buffer_size,
        ),
    )
}

pub(crate) struct Dispatcher<T, C, E> {
    publish: T,
    inner: Rc<Inner<C>>,
    max_buffer_size: usize,
    _t: PhantomData<E>,
}

struct Inner<C> {
    control: Pipeline<C>,
    sink: Rc<MqttShared>,
    payload: Cell<Option<PlSender>>,
    inflight: RefCell<HashSet<NonZeroU16>>,
}

impl<T, C, E> Dispatcher<T, C, E>
where
    T: Service<Publish, Response = Either<(), Publish>, Error = E>,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>,
{
    pub(crate) fn new(
        sink: Rc<MqttShared>,
        publish: T,
        control: C,
        max_buffer_size: usize,
    ) -> Self { panic!("STUB: not implemented") }
}

impl<C> Inner<C> {
    fn drop_payload<PErr>(&self, err: &PErr)
    where
        PErr: Clone,
        PayloadError: From<PErr>,
    { panic!("STUB: not implemented") }
}

impl<T, C, E> Service<Decoded> for Dispatcher<T, C, E>
where
    T: Service<Publish, Response = Either<(), Publish>, Error = E> + 'static,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>
        + 'static,
    E: 'static,
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
        packet: Decoded,
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
    T: Service<Publish, Response = Either<(), Publish>, Error = E>,
    C: Service<ProtocolMessage, Response = ProtocolMessageAck, Error = DispatcherError<E>>,
{ panic!("STUB: not implemented") }

impl<C> Inner<C> {
    async fn control<E>(
        &self,
        msg: ProtocolMessage,
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
    use ntex_util::future::{Ready, lazy};
    use ntex_util::time::{Seconds, sleep};

    use super::*;
    use crate::v3::{QoS, codec::Decoded};

    #[ntex::test]
    async fn test_dup_packet_id() {
        let io = Io::new(IoTest::create().0, SharedCfg::new("DBG"));
        let codec = codec::Codec::default();
        let shared = Rc::new(MqttShared::new(io.get_ref(), codec, false, Rc::default()));

        let disp = Pipeline::new(Dispatcher::<_, _, ()>::new(
            shared.clone(),
            fn_service(|_| async {
                sleep(Seconds(10)).await;
                Ok(Either::Left(()))
            }),
            fn_service(|_| {
                Ready::Ok(ProtocolMessageAck { result: ProtocolMessageKind::Nothing })
            }),
            32 * 1024,
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
        let err = f.await.err().unwrap();
        match err {
            DispatcherError::Protocol(msg) => {
                assert!(
                    format!("{msg}")
                        .contains("PUBLISH received with packet id that is already in use")
                );
            }
            DispatcherError::Service(()) => panic!(),
        }
    }
}
