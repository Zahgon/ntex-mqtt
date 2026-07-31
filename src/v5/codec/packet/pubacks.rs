use std::num::NonZeroU16;

use ntex_bytes::{Buf, BufMut, BytePages, ByteString, Bytes};

use super::ack_props;
use crate::error::{DecodeError, EncodeError};
use crate::utils::{Decode, Encode};
use crate::v5::codec::{UserProperties, encode};

const HEADER_LEN: u32 = 2 + 1; 

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PublishAck {
    
    pub packet_id: NonZeroU16,
    pub reason_code: PublishAckReason,
    pub properties: UserProperties,
    pub reason_string: Option<ByteString>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PublishAck2 {
    
    pub packet_id: NonZeroU16,
    pub reason_code: PublishAck2Reason,
    pub properties: UserProperties,
    pub reason_string: Option<ByteString>,
}

prim_enum! {
    
    pub enum PublishAckReason {
        Success = 0,
        NoMatchingSubscribers = 16,
        UnspecifiedError = 128,
        ImplementationSpecificError = 131,
        NotAuthorized = 135,
        TopicNameInvalid = 144,
        PacketIdentifierInUse = 145,
        QuotaExceeded = 151,
        PayloadFormatInvalid = 153
    }
}

prim_enum! {
    
    pub enum PublishAck2Reason {
        Success = 0,
        PacketIdNotFound = 146
    }
}

impl PublishAck {
    pub(crate) fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Default for PublishAck {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl PublishAck2 {
    pub(crate) fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Default for PublishAck2 {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl encode::EncodeLtd for PublishAck {
    fn encoded_size(&self, limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl encode::EncodeLtd for PublishAck2 {
    fn encoded_size(&self, limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(b"\xFF\xFF\x00\x00", 65535, PublishAckReason::Success, vec![], None; "success_empty")]
    #[test_case(b"\x00\x01", 1, PublishAckReason::Success, vec![], None; "success_no_reason")]
    #[test_case(b"\x01\x01\x00", 257, PublishAckReason::Success, vec![], None; "success_no_prop_len")]
    #[test_case(b"\x00\x01\x87", 1, PublishAckReason::NotAuthorized, vec![], None; "no_success_no_prop_len")]
    #[test_case(b"\x00\x01\x83\x00", 1, PublishAckReason::ImplementationSpecificError, vec![], None; "no_success_min")]
    #[test_case(b"\x00\xFF\x80\x0D\x26\x00\x01a\x00\x01b\x1F\x00\x03123", 255, PublishAckReason::UnspecifiedError, vec![("a", "b")], Some("123"); "all_out")]
    fn puback_decode_success(
        input: &'static [u8],
        packet_id: u16,
        reason_code: PublishAckReason,
        properties: Vec<(&'static str, &'static str)>,
        reason_string: Option<&'static str>,
    ) {
        let mut input = input.into();
        let result = PublishAck::decode(&mut input);
        assert_eq!(
            result,
            Ok(PublishAck {
                packet_id: packet_id.try_into().unwrap(),
                reason_code,
                properties: properties.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
                reason_string: reason_string.map(Into::into)
            })
        );
        assert_eq!(input.len(), 0);
    }

    #[test_case(b"\x00\x00", DecodeError::MalformedPacket; "packet_id_zero")]
    #[test_case(b"\x00\x01\x00\x01", DecodeError::InvalidLength; "properties_promised")]
    fn puback_decode_must_fail(input: &'static [u8], error: DecodeError) {
        let mut input = input.into();
        let result = PublishAck::decode(&mut input);
        assert_eq!(result, Err(error));
    }
}
