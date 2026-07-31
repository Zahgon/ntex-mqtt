use std::{num::NonZeroU16, num::NonZeroU32};

use ntex_bytes::{Buf, BufMut, BytePages, ByteString, Bytes, BytesMut};

use crate::error::{DecodeError, EncodeError};
use crate::types::{QoS, packet_type};
use crate::utils::{self, Decode, Encode, Property, write_variable_length};
use crate::v5::codec::{UserProperties, encode, property_type as pt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Publish {
    
    pub dup: bool,
    pub retain: bool,
    
    pub qos: QoS,
    
    pub packet_id: Option<NonZeroU16>,
    pub topic: ByteString,
    pub payload_size: u32,
    pub properties: PublishProperties,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct PublishProperties {
    pub topic_alias: Option<NonZeroU16>,
    pub correlation_data: Option<Bytes>,
    pub message_expiry_interval: Option<NonZeroU32>,
    pub content_type: Option<ByteString>,
    pub user_properties: UserProperties,
    pub is_utf8_payload: bool,
    pub response_topic: Option<ByteString>,
    pub subscription_ids: Vec<NonZeroU32>,
}

impl Default for Publish {
    fn default() -> Publish { panic!("STUB: not implemented") }
}

impl Publish {
    pub(crate) fn decode(
        src: &mut Bytes,
        packet_flags: u8,
        payload_size: u32,
    ) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }

    pub(crate) fn packet_header_size(
        src: &BytesMut,
        packet_flags: u8,
    ) -> Result<Option<u32>, DecodeError> { panic!("STUB: not implemented") }
}

fn parse_publish_properties(src: &mut Bytes) -> Result<PublishProperties, DecodeError> { panic!("STUB: not implemented") }

impl encode::EncodeLtd for Publish {
    fn encoded_size(&self, limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl encode::EncodeLtd for PublishProperties {
    fn encoded_size(&self, _limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}
