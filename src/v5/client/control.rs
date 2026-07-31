use ntex_bytes::{ByteString, Bytes};

use crate::{error, payload::Payload, v5::codec, v5::control::Pkt};

pub use crate::v5::control::{Disconnect, Ping, ProtocolMessageAck, PublishRelease};

#[derive(Debug)]
pub enum ProtocolMessage {
    
    Publish(Publish),
    
    PublishRelease(PublishRelease),
    
    Disconnect(Disconnect),
    
    Ping(Ping),
}

impl ProtocolMessage {
    pub(super) fn publish(pkt: codec::Publish, pl: Payload, size: u32) -> Self { panic!("STUB: not implemented") }

    pub(super) fn pubrel(pkt: codec::PublishAck2, size: u32) -> Self { panic!("STUB: not implemented") }

    pub(super) fn dis(pkt: codec::Disconnect, size: u32) -> Self { panic!("STUB: not implemented") }

    pub fn disconnect(&self, pkt: codec::Disconnect) -> ProtocolMessageAck { panic!("STUB: not implemented") }

    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[derive(Debug)]
pub struct Publish(codec::Publish, Payload, u32);

impl Publish {
    #[inline]
    
    pub fn packet(&self) -> &codec::Publish { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet_mut(&mut self) -> &mut codec::Publish { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn payload_size(&self) -> usize { panic!("STUB: not implemented") }

    #[inline]
    
    pub async fn read(&self) -> Result<Option<Bytes>, error::PayloadError> { panic!("STUB: not implemented") }

    #[inline]
    
    pub async fn read_all(&self) -> Result<Bytes, error::PayloadError> { panic!("STUB: not implemented") }

    #[inline]
    pub fn ack_qos0(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }

    #[inline]
    pub fn ack(self, reason_code: codec::PublishAckReason) -> ProtocolMessageAck { panic!("STUB: not implemented") }

    #[inline]
    pub fn ack_with(
        self,
        reason_code: codec::PublishAckReason,
        properties: codec::UserProperties,
        reason_string: Option<ByteString>,
    ) -> ProtocolMessageAck { panic!("STUB: not implemented") }

    pub fn into_inner(
        self,
        reason_code: codec::PublishAckReason,
    ) -> (ProtocolMessageAck, codec::Publish) { panic!("STUB: not implemented") }
}
