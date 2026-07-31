#![allow(clippy::type_complexity)]
use std::{cell::Cell, cell::RefCell, collections::VecDeque, fmt, num, rc::Rc};

use ntex_bytes::{BytePages, Bytes, BytesMut};
use ntex_codec::{Decoder, Encoder};
use ntex_io::IoRef;
use ntex_util::{HashSet, channel::pool};

use crate::v5::codec::{self, Decoded, Encoded, Packet, Publish};
use crate::{QoS, error, error::SendPacketError, payload::PlSender, types::packet_type};

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub(crate) struct Flags: u8 {
        const WRB_ENABLED     = 0b0000_0001; 
        const ON_PUBLISH_ACK  = 0b0000_0010; 

        const QOS_ATLEAST     = 0b0000_0100; 
        const QOS_EXACTLY     = 0b0000_1000; 

        const ZERO_SES_EXPIRY = 0b0001_0000; 

        const DISCONNECT      = 0b0010_0000; 
        const DISCONNECT_RECV = 0b0100_0000; 
        const STOPPED         = 0b1000_0000; 
    }
}

pub struct MqttShared {
    io: IoRef,
    cap: Cell<usize>,
    receive_max: Cell<u16>,
    topic_alias_max: Cell<u16>,
    inflight_idx: Cell<u16>,
    queues: RefCell<MqttSharedQueues>,
    encode_error: Cell<Option<error::EncodeError>>,
    streaming_waiter: Cell<Option<pool::Sender<()>>>,
    streaming_remaining: Cell<Option<num::NonZeroU32>>,
    on_publish_ack: Cell<Option<Box<dyn Fn(codec::PublishAck, bool)>>>,
    pub(super) payload: Cell<Option<PlSender>>,
    pub(super) flags: Cell<Flags>,
    pub(super) pool: Rc<MqttSinkPool>,
    pub(super) codec: codec::Codec,
}

impl fmt::Debug for MqttShared {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Debug)]
pub(super) struct MqttSharedQueues {
    inflight: VecDeque<(num::NonZeroU16, Option<pool::Sender<Ack>>, AckType)>,
    inflight_ids: HashSet<num::NonZeroU16>,
    waiters: VecDeque<pool::Sender<()>>,
    rx: Option<pool::Receiver<Ack>>,
}

pub(super) struct MqttSinkPool {
    queue: pool::Pool<Ack>,
    pub(super) waiters: pool::Pool<()>,
}

impl Default for MqttSinkPool {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl MqttShared {
    pub(super) fn new(io: IoRef, codec: codec::Codec, pool: Rc<MqttSinkPool>) -> Self { panic!("STUB: not implemented") }

    pub(super) fn tag(&self) -> &'static str { panic!("STUB: not implemented") }

    pub(super) fn credit(&self) -> usize { panic!("STUB: not implemented") }

    pub(super) fn receive_max(&self) -> u16 { panic!("STUB: not implemented") }

    pub(super) fn topic_alias_max(&self) -> u16 { panic!("STUB: not implemented") }

    pub(super) fn max_qos(&self) -> QoS { panic!("STUB: not implemented") }

    pub(super) fn set_receive_max(&self, val: u16) { panic!("STUB: not implemented") }

    pub(super) fn set_topic_alias_max(&self, val: u16) { panic!("STUB: not implemented") }

    pub(super) fn set_max_qos(&self, val: QoS) { panic!("STUB: not implemented") }

    pub(super) fn is_zero_session_expiry(&self) -> bool { panic!("STUB: not implemented") }

    pub(super) fn set_zero_session_expiry(&self) { panic!("STUB: not implemented") }

    pub(super) fn close(&self, pkt: Option<codec::Disconnect>) { panic!("STUB: not implemented") }

    pub(super) fn force_close(&self) { panic!("STUB: not implemented") }

    pub(super) fn streaming_dropped(&self) { panic!("STUB: not implemented") }

    pub(super) fn is_closed(&self) -> bool { panic!("STUB: not implemented") }

    pub(super) fn is_streaming(&self) -> bool { panic!("STUB: not implemented") }

    pub(super) fn is_ready(&self) -> bool { panic!("STUB: not implemented") }

    pub(super) fn is_disconnect_sent(&self) -> bool { panic!("STUB: not implemented") }

    pub(super) fn set_disconnect_recv(&self) { panic!("STUB: not implemented") }

    pub(super) fn is_disconnect_recv(&self) -> bool { panic!("STUB: not implemented") }

    pub(super) fn set_publish_id(&self, pkt: &mut Publish) -> num::NonZeroU16 { panic!("STUB: not implemented") }

    pub(super) fn next_id(&self) -> num::NonZeroU16 { panic!("STUB: not implemented") }

    pub(super) fn set_cap(&self, cap: usize) { panic!("STUB: not implemented") }

    pub(super) fn set_publish_ack(&self, f: Box<dyn Fn(codec::PublishAck, bool)>) { panic!("STUB: not implemented") }

    pub(super) fn drop_sink(&self, io: bool) { panic!("STUB: not implemented") }

    pub(super) fn drop_payload<E>(&self, err: &E)
    where
        E: Clone,
        error::PayloadError: From<E>,
    { panic!("STUB: not implemented") }

    fn clear_queues(&self) { panic!("STUB: not implemented") }

    pub(super) fn enable_wr_backpressure(&self) { panic!("STUB: not implemented") }

    pub(super) fn disable_wr_backpressure(&self) { panic!("STUB: not implemented") }

    pub(super) async fn want_payload_stream(&self) -> Result<(), SendPacketError> { panic!("STUB: not implemented") }

    fn check_streaming(&self) -> Result<(), error::EncodeError> { panic!("STUB: not implemented") }

    fn enable_streaming(&self, pkt: &Publish, payload: Option<&Bytes>) { panic!("STUB: not implemented") }

    pub(super) fn encode_packet(&self, pkt: codec::Packet) -> Result<(), error::EncodeError> { panic!("STUB: not implemented") }

    pub(super) fn encode_publish(
        &self,
        pkt: Publish,
        payload: Option<Bytes>,
    ) -> Result<(), error::EncodeError> { panic!("STUB: not implemented") }

    pub(super) fn encode_publish_payload(
        &self,
        payload: Bytes,
    ) -> Result<bool, error::EncodeError> { panic!("STUB: not implemented") }

    pub(super) fn pkt_ack(&self, ack: Ack) -> Result<(), error::ProtocolError> { panic!("STUB: not implemented") }

    fn pkt_ack_inner(&self, pkt: Ack) -> Result<(), error::ProtocolError> { panic!("STUB: not implemented") }

    pub(super) fn wait_response(
        &self,
        id: num::NonZeroU16,
        ack: AckType,
    ) -> Result<pool::Receiver<Ack>, SendPacketError> { panic!("STUB: not implemented") }

    pub(super) fn wait_publish_response(
        &self,
        id: num::NonZeroU16,
        ack: AckType,
        pkt: Publish,
        payload: Option<Bytes>,
    ) -> Result<pool::Receiver<Ack>, SendPacketError> { panic!("STUB: not implemented") }

    pub(super) fn wait_publish_response_no_block(
        &self,
        id: num::NonZeroU16,
        ack: AckType,
        pkt: Publish,
        payload: Option<Bytes>,
    ) -> Result<(), SendPacketError> { panic!("STUB: not implemented") }

    pub(super) fn wait_readiness(&self) -> Option<pool::Receiver<()>> { panic!("STUB: not implemented") }

    pub(super) fn release_publish(
        &self,
        pkt: codec::PublishAck2,
    ) -> Result<pool::Receiver<Ack>, SendPacketError> { panic!("STUB: not implemented") }
}

impl Encoder for MqttShared {
    type Item = Encoded;
    type Error = error::EncodeError;

    #[inline]
    fn encodev(&self, item: Self::Item, dst: &mut BytePages) -> Result<(), Self::Error> { panic!("STUB: not implemented") }
}

impl Decoder for MqttShared {
    type Item = Decoded;
    type Error = error::DecodeError;

    #[inline]
    fn decode(&self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> { panic!("STUB: not implemented") }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum AckType {
    Publish,
    Receive,
    Complete,
    Subscribe,
    Unsubscribe,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Ack {
    Publish(codec::PublishAck),
    Receive(codec::PublishAck),
    Complete(codec::PublishAck2),
    Subscribe(codec::SubscribeAck),
    Unsubscribe(codec::UnsubscribeAck),
}

impl Ack {
    pub(super) fn packet_type(&self) -> u8 { panic!("STUB: not implemented") }

    pub(super) fn packet_id(&self) -> num::NonZeroU16 { panic!("STUB: not implemented") }

    pub(super) fn publish(self) -> codec::PublishAck { panic!("STUB: not implemented") }

    pub(super) fn receive(self) -> codec::PublishAck { panic!("STUB: not implemented") }

    pub(super) fn subscribe(self) -> codec::SubscribeAck { panic!("STUB: not implemented") }

    pub(super) fn unsubscribe(self) -> codec::UnsubscribeAck { panic!("STUB: not implemented") }

    pub(super) fn is_match(&self, tp: AckType) -> bool { panic!("STUB: not implemented") }
}

impl AckType {
    pub(super) fn expected_str(self) -> &'static str { panic!("STUB: not implemented") }
}
