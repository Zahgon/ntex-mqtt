use std::{mem, num::NonZeroU16};

use ntex_bytes::{ByteString, Bytes};
use ntex_router::Path;

use crate::{error::PayloadError, payload::Payload, v3::codec};

#[derive(Debug)]

pub struct Publish {
    pkt: codec::Publish,
    pkt_size: u32,
    topic: Path<ByteString>,
    payload: Payload,
}

impl Publish {
    
    pub fn new(pkt: codec::Publish, payload: Payload, pkt_size: u32) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn dup(&self) -> bool { panic!("STUB: not implemented") }

    #[inline]
    pub fn retain(&self) -> bool { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn qos(&self) -> codec::QoS { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn publish_topic(&self) -> &str { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn id(&self) -> Option<NonZeroU16> { panic!("STUB: not implemented") }

    #[inline]
    pub fn topic(&self) -> &Path<ByteString> { panic!("STUB: not implemented") }

    #[inline]
    pub fn topic_mut(&mut self) -> &mut Path<ByteString> { panic!("STUB: not implemented") }

    #[inline]
    pub fn packet(&self) -> &codec::Publish { panic!("STUB: not implemented") }

    #[inline]
    pub fn packet_mut(&mut self) -> &mut codec::Publish { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn payload_size(&self) -> usize { panic!("STUB: not implemented") }

    #[inline]
    
    pub async fn read(&self) -> Result<Option<Bytes>, PayloadError> { panic!("STUB: not implemented") }

    #[inline]
    
    pub async fn read_all(&self) -> Result<Bytes, PayloadError> { panic!("STUB: not implemented") }

    pub fn take_payload(&mut self) -> Payload { panic!("STUB: not implemented") }

    pub(super) fn into_inner(self) -> (codec::Publish, Payload, u32) { panic!("STUB: not implemented") }
}
