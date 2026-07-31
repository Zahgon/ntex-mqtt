use std::num::NonZeroU16;

use ntex_bytes::Bytes;

use crate::payload::Payload;
pub use crate::v3::control::{Disconnect, Ping, ProtocolMessageAck, PublishRelease};
use crate::v3::{codec, control::ProtocolMessageKind, error};

#[derive(Debug)]
pub enum ProtocolMessage {
    
    Publish(Publish),
    
    PublishRelease(PublishRelease),
    
    Ping(Ping),
}

impl ProtocolMessage {
    pub(super) fn publish(pkt: codec::Publish, pl: Payload, size: u32) -> Self { panic!("STUB: not implemented") }

    pub(super) fn pubrel(packet_id: NonZeroU16) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub(super) fn disconnect() -> ProtocolMessageAck { panic!("STUB: not implemented") }

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
    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }

    #[inline]
    pub fn into_inner(self) -> (ProtocolMessageAck, codec::Publish) { panic!("STUB: not implemented") }
}
