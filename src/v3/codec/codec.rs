use std::{cell::Cell, cmp::min, num::NonZeroU32};

use ntex_bytes::{Buf, BytePages, Bytes, BytesMut};
use ntex_codec::{Decoder, Encoder};

use crate::error::{DecodeError, EncodeError};
use crate::types::{FixedHeader, QoS, packet_type};
use crate::utils::decode_variable_length;

use super::{Decoded, Encoded, Publish, decode, encode};

#[derive(Debug, Clone)]

pub struct Codec {
    state: Cell<DecodeState>,
    max_size: Cell<u32>,
    min_chunk_size: Cell<u32>,
    encoding_payload: Cell<Option<NonZeroU32>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DecodeState {
    FrameHeader,
    Frame(FixedHeader),
    PublishHeader(FixedHeader),
    PublishPayload(u32),
}

impl Codec {
    
    pub fn new() -> Self { panic!("STUB: not implemented") }

    pub fn set_max_size(&self, size: u32) { panic!("STUB: not implemented") }

    pub fn set_min_chunk_size(&self, size: u32) { panic!("STUB: not implemented") }
}

impl Default for Codec {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl Decoder for Codec {
    type Item = Decoded;
    type Error = DecodeError;

    #[allow(clippy::too_many_lines)]
    fn decode(&self, src: &mut BytesMut) -> Result<Option<Self::Item>, DecodeError> { panic!("STUB: not implemented") }
}

impl Encoder for Codec {
    type Item = Encoded;
    type Error = EncodeError;

    fn encodev(&self, item: Self::Item, dst: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntex_bytes::{ByteString, Bytes};

    #[test]
    fn test_max_size() {
        let codec = Codec::new();
        codec.set_max_size(5);

        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"\0\x09");
        assert_eq!(
            codec.decode(&mut buf),
            Err(DecodeError::MaxSizeExceeded { size: 9, max_size: 5 })
        );
    }

    #[test]
    fn test_packet() {
        let codec = Codec::new();
        let mut buf = BytePages::default();

        let pkt = Publish {
            dup: false,
            retain: false,
            qos: QoS::AtMostOnce,
            topic: ByteString::from_static("/test"),
            packet_id: None,
            payload_size: 260 * 1024,
        };
        let payload = Bytes::from(Vec::from("a".repeat(260 * 1024)));
        codec.encodev(Encoded::Publish(pkt.clone(), Some(payload)), &mut buf).unwrap();

        let Decoded::Publish(pkt2, _, _) =
            codec.decode(&mut BytesMut::from(buf.freeze())).unwrap().unwrap()
        else {
            panic!()
        };
        assert_eq!(pkt, pkt2);
    }
}
