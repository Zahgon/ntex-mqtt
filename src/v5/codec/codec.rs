use std::{cell::Cell, cmp::min, fmt, num::NonZeroU32};

use ntex_bytes::{Buf, BytePages, Bytes, BytesMut};
use ntex_codec::{Decoder, Encoder};

use crate::error::{DecodeError, EncodeError};
use crate::types::{FixedHeader, MAX_PACKET_SIZE, packet_type};
use crate::utils::decode_variable_length;

use super::{Decoded, Encoded};
use super::{Packet, decode::decode_packet, encode::EncodeLtd, packet::Publish};

pub struct Codec {
    state: Cell<DecodeState>,
    max_in_size: Cell<u32>,
    max_out_size: Cell<u32>,
    min_chunk_size: Cell<u32>,
    flags: Cell<CodecFlags>,
    encoding_payload: Cell<Option<NonZeroU32>>,
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CodecFlags: u8 {
        const NO_PROBLEM_INFO = 0b0000_0001;
        const NO_RETAIN       = 0b0000_0010;
        const NO_SUB_IDS      = 0b0000_1000;
    }
}

#[derive(Debug, Clone, Copy)]
enum DecodeState {
    FrameHeader,
    Frame(FixedHeader),
    PublishHeader(FixedHeader),
    PublishProperties(u32, FixedHeader),
    PublishPayload(u32),
}

impl Codec {
    
    pub fn new() -> Self { panic!("STUB: not implemented") }

    pub fn set_min_chunk_size(&self, size: u32) { panic!("STUB: not implemented") }

    pub fn max_inbound_size(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn max_outbound_size(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn set_max_inbound_size(&self, size: u32) { panic!("STUB: not implemented") }

    pub fn set_max_outbound_size(&self, mut size: u32) { panic!("STUB: not implemented") }

    pub(crate) fn retain_available(&self) -> bool { panic!("STUB: not implemented") }

    pub(crate) fn sub_ids_available(&self) -> bool { panic!("STUB: not implemented") }

    pub(crate) fn set_retain_available(&self, val: bool) { panic!("STUB: not implemented") }

    pub(crate) fn set_sub_ids_available(&self, val: bool) { panic!("STUB: not implemented") }
}

impl Default for Codec {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl Decoder for Codec {
    type Item = super::Decoded;
    type Error = DecodeError;

    #[allow(clippy::too_many_lines)]
    fn decode(&self, src: &mut BytesMut) -> Result<Option<Self::Item>, DecodeError> { panic!("STUB: not implemented") }
}

impl Encoder for Codec {
    type Item = Encoded;
    type Error = EncodeError;

    fn encodev(&self, mut item: Self::Item, dst: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Clone for Codec {
    fn clone(&self) -> Self { panic!("STUB: not implemented") }
}

impl fmt::Debug for Codec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_size() {
        let codec = Codec::new();
        codec.set_max_inbound_size(5);
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"\0\x09");
        assert_eq!(
            codec.decode(&mut buf).err(),
            Some(DecodeError::MaxSizeExceeded { size: 9, max_size: 5 })
        );
    }
}
