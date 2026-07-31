use std::{cell::Cell, fmt, future::Future, future::ready, num::NonZeroU16, rc::Rc};

use ntex_bytes::{ByteString, Bytes};
use ntex_util::{channel::pool, future::Either, future::Ready};

use crate::v3::shared::{Ack, AckType, MqttShared};
use crate::v3::{codec, error::SendPacketError};
use crate::{error::EncodeError, types::QoS};

pub struct MqttSink(Rc<MqttShared>);

impl Clone for MqttSink {
    fn clone(&self) -> Self { panic!("STUB: not implemented") }
}

impl MqttSink {
    pub(crate) fn new(state: Rc<MqttShared>) -> Self { panic!("STUB: not implemented") }

    pub(super) fn shared(&self) -> Rc<MqttShared> { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn is_open(&self) -> bool { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn is_ready(&self) -> bool { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn credit(&self) -> usize { panic!("STUB: not implemented") }

    pub fn ready(&self) -> impl Future<Output = bool> {
        if self.0.is_closed() {
            Either::Left(ready(false))
        } else {
            self.0.wait_readiness().map_or_else(
                || Either::Left(ready(true)),
                |rx| Either::Right(async move { rx.await.is_ok() }),
            )
        }
    }

    #[inline]
    
    pub fn close(&self) { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn force_close(&self) { panic!("STUB: not implemented") }

    #[inline]
    
    pub(super) fn ping(&self) -> bool { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn publish<U>(&self, topic: U) -> PublishBuilder
    where
        ByteString: From<U>,
    { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn publish_pkt(&self, packet: codec::Publish) -> PublishBuilder { panic!("STUB: not implemented") }

    pub fn publish_ack_cb<F>(&self, f: F)
    where
        F: Fn(NonZeroU16, bool) + 'static,
    { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn subscribe(&self) -> SubscribeBuilder { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn unsubscribe(&self) -> UnsubscribeBuilder { panic!("STUB: not implemented") }
}

impl fmt::Debug for MqttSink {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct PublishBuilder {
    packet: codec::Publish,
    shared: Rc<MqttShared>,
}

impl fmt::Debug for PublishBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl PublishBuilder {
    #[inline]
    #[must_use]
    
    pub fn packet_id(mut self, id: u16) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn dup(mut self, val: bool) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn retain(mut self) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn size(&self, payload_size: usize) -> u32 { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn send_at_most_once(mut self, payload: Bytes) -> Result<(), SendPacketError> { panic!("STUB: not implemented") }

    pub fn stream_at_most_once(
        mut self,
        size: u32,
    ) -> Result<StreamingPayload, SendPacketError> { panic!("STUB: not implemented") }

    pub fn send_at_least_once(
        mut self,
        payload: Bytes,
    ) -> impl Future<Output = Result<(), SendPacketError>> {
        if self.shared.is_closed() {
            Either::Right(Ready::Err(SendPacketError::Disconnected))
        } else {
            self.packet.qos = codec::QoS::AtLeastOnce;
            self.packet.payload_size = payload.len() as u32;

            if let Some(rx) = self.shared.wait_readiness() {
                Either::Left(Either::Left(async move {
                    if rx.await.is_err() {
                        return Err(SendPacketError::Disconnected);
                    }
                    self.send_at_least_once_inner(payload).await
                }))
            } else {
                Either::Left(Either::Right(self.send_at_least_once_inner(payload)))
            }
        }
    }

    pub fn send_at_least_once_no_block(
        mut self,
        payload: Bytes,
    ) -> Result<(), SendPacketError> { panic!("STUB: not implemented") }

    fn send_at_least_once_inner(
        mut self,
        payload: Bytes,
    ) -> impl Future<Output = Result<(), SendPacketError>> {
        let idx = self.shared.set_publish_id(&mut self.packet);
        log::trace!("Publish (QoS1) to {:#?}", self.packet);

        let rx = self.shared.wait_publish_response(
            idx,
            AckType::Publish,
            self.packet,
            Some(payload),
        );
        async move { rx?.await.map(|_| ()).map_err(|_| SendPacketError::Disconnected) }
    }

    pub fn send_exactly_once(
        mut self,
        payload: Bytes,
    ) -> impl Future<Output = Result<PublishReceived, SendPacketError>> {
        if self.shared.is_closed() {
            Either::Right(Ready::Err(SendPacketError::Disconnected))
        } else {
            self.packet.qos = codec::QoS::ExactlyOnce;
            self.packet.payload_size = payload.len() as u32;

            if let Some(rx) = self.shared.wait_readiness() {
                Either::Left(Either::Left(async move {
                    if rx.await.is_err() {
                        return Err(SendPacketError::Disconnected);
                    }
                    self.send_exactly_once_inner(payload).await
                }))
            } else {
                Either::Left(Either::Right(self.send_exactly_once_inner(payload)))
            }
        }
    }

    fn send_exactly_once_inner(
        mut self,
        payload: Bytes,
    ) -> impl Future<Output = Result<PublishReceived, SendPacketError>> {
        let idx = self.shared.set_publish_id(&mut self.packet);
        log::trace!("Publish (QoS2) to {:#?}", self.packet);

        let rx = self.shared.wait_publish_response(
            idx,
            AckType::Receive,
            self.packet,
            Some(payload),
        );
        async move {
            rx?.await
                .map(move |_| PublishReceived { packet_id: Some(idx), shared: self.shared })
                .map_err(|_| SendPacketError::Disconnected)
        }
    }

    pub fn stream_at_least_once(
        mut self,
        size: u32,
    ) -> (impl Future<Output = Result<(), SendPacketError>>, StreamingPayload) {
        let (tx, rx) = self.shared.pool.waiters.channel();
        let stream = StreamingPayload {
            rx: Cell::new(Some(rx)),
            shared: self.shared.clone(),
            inprocess: Cell::new(false),
        };

        if self.shared.is_closed() {
            (Either::Right(Ready::Err(SendPacketError::Disconnected)), stream)
        } else {
            self.packet.qos = QoS::AtLeastOnce;
            self.packet.payload_size = size;

            let fut = if let Some(rx) = self.shared.wait_readiness() {
                Either::Left(Either::Left(async move {
                    if rx.await.is_err() {
                        return Err(SendPacketError::Disconnected);
                    }
                    self.stream_at_least_once_inner(tx).await
                }))
            } else {
                Either::Left(Either::Right(self.stream_at_least_once_inner(tx)))
            };
            (fut, stream)
        }
    }

    async fn stream_at_least_once_inner(
        mut self,
        tx: pool::Sender<()>,
    ) -> Result<(), SendPacketError> { panic!("STUB: not implemented") }
}

pub struct PublishReceived {
    packet_id: Option<NonZeroU16>,
    shared: Rc<MqttShared>,
}

impl fmt::Debug for PublishReceived {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl PublishReceived {
    
    pub async fn release(mut self) -> Result<(), SendPacketError> { panic!("STUB: not implemented") }
}

impl Drop for PublishReceived {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

pub struct SubscribeBuilder {
    id: Option<NonZeroU16>,
    shared: Rc<MqttShared>,
    topic_filters: Vec<(ByteString, codec::QoS)>,
}

impl fmt::Debug for SubscribeBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl SubscribeBuilder {
    #[inline]
    #[must_use]
    
    pub fn packet_id(mut self, id: u16) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn topic_filter(mut self, filter: ByteString, qos: codec::QoS) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn size(&self) -> u32 { panic!("STUB: not implemented") }

    pub async fn send(self) -> Result<Vec<codec::SubscribeReturnCode>, SendPacketError> { panic!("STUB: not implemented") }
}

pub struct UnsubscribeBuilder {
    id: Option<NonZeroU16>,
    shared: Rc<MqttShared>,
    topic_filters: Vec<ByteString>,
}

impl fmt::Debug for UnsubscribeBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl UnsubscribeBuilder {
    #[inline]
    #[must_use]
    
    pub fn packet_id(mut self, id: u16) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn topic_filter(mut self, filter: ByteString) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn size(&self) -> u32 { panic!("STUB: not implemented") }

    pub async fn send(self) -> Result<(), SendPacketError> { panic!("STUB: not implemented") }
}

pub struct StreamingPayload {
    shared: Rc<MqttShared>,
    rx: Cell<Option<pool::Receiver<()>>>,
    inprocess: Cell<bool>,
}

impl fmt::Debug for StreamingPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl Drop for StreamingPayload {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}

impl StreamingPayload {
    
    pub async fn send(&self, chunk: Bytes) -> Result<(), SendPacketError> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use ntex_io::{Io, testing::IoTest};
    use ntex_service::cfg::SharedCfg;

    use super::*;
    use crate::v3::shared::MqttShared;

    #[ntex::test]
    async fn test_debug() {
        let io = Io::new(IoTest::create().0, SharedCfg::new("test"));
        let codec = codec::Codec::default();
        let shared = Rc::new(MqttShared::new(io.get_ref(), codec, true, Rc::default()));
        let sink = MqttSink::new(shared);

        assert!(format!("{sink:?}").contains("MqttSink"));

        let pb = sink.publish("test/topic");
        assert!(format!("{pb:?}").contains("PublishBuilder"));

        let sb = sink.subscribe();
        assert!(format!("{sb:?}").contains("SubscribeBuilder"));

        let ub = sink.unsubscribe();
        assert!(format!("{ub:?}").contains("UnsubscribeBuilder"));
    }
}
