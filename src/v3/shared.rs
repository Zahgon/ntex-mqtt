#![allow(clippy::type_complexity)]
use std::{cell::Cell, cell::RefCell, collections::VecDeque, fmt, num, rc::Rc};

use ntex_bytes::{BytePages, Bytes, BytesMut};
use ntex_codec::{Decoder, Encoder};
use ntex_io::IoRef;
use ntex_util::{HashSet, channel::pool};

use crate::error::{DecodeError, EncodeError, PayloadError, ProtocolError, SendPacketError};
use crate::v3::codec::{self, Encoded, Publish};
use crate::{payload::PlSender, types::packet_type};

#[derive(Debug)]
pub(super) enum Ack {
    Publish(num::NonZeroU16),
    Receive(num::NonZeroU16),
    Complete(num::NonZeroU16),
    Subscribe { packet_id: num::NonZeroU16, status: Vec<codec::SubscribeReturnCode> },
    Unsubscribe(num::NonZeroU16),
}

#[derive(Copy, Clone, Debug)]
pub(super) enum AckType {
    Publish,
    Receive,
    Complete,
    Subscribe,
    Unsubscribe,
}

pub(super) struct MqttSinkPool {
    queue: pool::Pool<Ack>,
    pub(super) waiters: pool::Pool<()>,
}

impl Default for MqttSinkPool {
    fn default() -> Self { panic!("STUB: not implemented") }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Flags: u8 {
        const CLIENT          = 0b0000_0001;
        const WRB_ENABLED     = 0b0000_0010; 
        const ON_PUBLISH_ACK  = 0b0000_0100; 

        const DISCONNECT      = 0b0010_0000; 
        const STOPPED         = 0b1000_0000; 
    }
}

pub struct MqttShared {
    io: IoRef,
    cap: Cell<usize>,
    queues: RefCell<MqttSharedQueues>,
    inflight_idx: Cell<u16>,
    flags: Cell<Flags>,
    encode_error: Cell<Option<EncodeError>>,
    streaming_waiter: Cell<Option<pool::Sender<()>>>,
    streaming_remaining: Cell<Option<num::NonZeroU32>>,
    on_publish_ack: Cell<Option<Box<dyn Fn(num::NonZeroU16, bool)>>>,
    pub(super) payload: Cell<Option<PlSender>>,
    pub(super) codec: codec::Codec,
    pub(super) pool: Rc<MqttSinkPool>,
}

impl fmt::Debug for MqttShared {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Debug)]
struct MqttSharedQueues {
    inflight: VecDeque<(num::NonZeroU16, Option<pool::Sender<Ack>>, AckType)>,
    inflight_ids: HashSet<num::NonZeroU16>,
    waiters: VecDeque<pool::Sender<()>>,
    rx: Option<pool::Receiver<Ack>>,
}

impl MqttShared {
    pub(super) fn new(
        io: IoRef,
        codec: codec::Codec,
        client: bool,
        pool: Rc<MqttSinkPool>,
    ) -> Self { panic!("STUB: not implemented") }

    pub(super) fn tag(&self) -> &'static str { panic!("STUB: not implemented") }

    pub(super) fn close(&self) { panic!("STUB: not implemented") }

    pub(super) fn force_close(&self) { panic!("STUB: not implemented") }

    pub(super) fn streaming_dropped(&self) { panic!("STUB: not implemented") }

    pub(super) fn drop_payload<E>(&self, err: &E)
    where
        E: Clone,
        PayloadError: From<E>,
    { panic!("STUB: not implemented") }

    pub(super) fn is_streaming(&self) -> bool { panic!("STUB: not implemented") }

    pub(super) fn is_closed(&self) -> bool { panic!("STUB: not implemented") }

    pub(super) fn is_ready(&self) -> bool { panic!("STUB: not implemented") }

    pub(super) fn is_disconnect_sent(&self) -> bool { panic!("STUB: not implemented") }

    pub(super) fn credit(&self) -> usize { panic!("STUB: not implemented") }

    pub(super) fn next_id(&self) -> num::NonZeroU16 { panic!("STUB: not implemented") }

    pub(super) fn set_publish_id(&self, pkt: &mut Publish) -> num::NonZeroU16 { panic!("STUB: not implemented") }

    pub(super) fn set_cap(&self, cap: usize) { panic!("STUB: not implemented") }

    pub(super) fn set_publish_ack(&self, f: Box<dyn Fn(num::NonZeroU16, bool)>) { panic!("STUB: not implemented") }

    pub(super) fn encode_packet(&self, pkt: codec::Packet) -> Result<(), EncodeError> { panic!("STUB: not implemented") }

    pub(super) fn encode_publish(
        &self,
        pkt: Publish,
        payload: Option<Bytes>,
    ) -> Result<(), EncodeError> { panic!("STUB: not implemented") }

    pub(super) fn encode_publish_payload(&self, payload: Bytes) -> Result<bool, EncodeError> { panic!("STUB: not implemented") }

    fn clear_queues(&self) { panic!("STUB: not implemented") }

    pub(super) fn enable_wr_backpressure(&self) { panic!("STUB: not implemented") }

    pub(super) fn disable_wr_backpressure(&self) { panic!("STUB: not implemented") }

    pub(super) async fn want_payload_stream(&self) -> Result<(), SendPacketError> { panic!("STUB: not implemented") }

    fn check_streaming(&self) -> Result<(), EncodeError> { panic!("STUB: not implemented") }

    fn enable_streaming(&self, pkt: &Publish, payload: Option<&Bytes>) { panic!("STUB: not implemented") }

    pub(super) fn pkt_ack(&self, ack: Ack) -> Result<(), ProtocolError> { panic!("STUB: not implemented") }

    fn pkt_ack_inner(&self, pkt: Ack) -> Result<(), ProtocolError> { panic!("STUB: not implemented") }

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
        id: num::NonZeroU16,
    ) -> Result<pool::Receiver<Ack>, SendPacketError> { panic!("STUB: not implemented") }
}

impl Encoder for MqttShared {
    type Item = Encoded;
    type Error = EncodeError;

    #[inline]
    fn encodev(&self, item: Self::Item, dst: &mut BytePages) -> Result<(), Self::Error> { panic!("STUB: not implemented") }
}

impl Decoder for MqttShared {
    type Item = codec::Decoded;
    type Error = DecodeError;

    #[inline]
    fn decode(&self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> { panic!("STUB: not implemented") }
}

impl Ack {
    pub(super) fn packet_type(&self) -> u8 { panic!("STUB: not implemented") }

    pub(super) fn packet_id(&self) -> num::NonZeroU16 { panic!("STUB: not implemented") }

    pub(super) fn subscribe(self) -> Vec<codec::SubscribeReturnCode> { panic!("STUB: not implemented") }

    pub(super) fn is_match(&self, tp: AckType) -> bool { panic!("STUB: not implemented") }
}

impl AckType {
    pub(super) fn expected_str(self) -> &'static str { panic!("STUB: not implemented") }
}
