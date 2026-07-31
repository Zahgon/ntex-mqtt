use ntex_bytes::BytesMut;
use ntex_codec::{Decoder, Encoder};

use crate::error::{DecodeError, EncodeError};
use crate::types::{MQTT, MQTT_LEVEL_3, MQTT_LEVEL_5, packet_type};
use crate::utils;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum ProtocolVersion {
    MQTT3,
    MQTT5,
}

#[derive(Debug)]
pub(super) struct VersionCodec;

impl Decoder for VersionCodec {
    type Item = ProtocolVersion;
    type Error = DecodeError;

    fn decode(&self, src: &mut BytesMut) -> Result<Option<Self::Item>, DecodeError> { panic!("STUB: not implemented") }
}

impl Encoder for VersionCodec {
    type Item = ProtocolVersion;
    type Error = EncodeError;

    fn encode(&self, _: Self::Item, _: &mut BytesMut) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_connect_packets() {
        let mut buf = BytesMut::from(
            b"\x10\x7f\x7f\x00\x04MQTT\x06\xC0\x00\x3C\x00\x0512345\x00\x04user\x00\x04pass"
                .as_ref(),
        );
        assert_eq!(Err(DecodeError::InvalidProtocol), VersionCodec.decode(&mut buf));

        let mut buf =
            BytesMut::from(b"\x10\x98\x02\0\x04MQTT\x04\xc0\0\x0f\0\x02d1\0|testhub.".as_ref());
        assert_eq!(ProtocolVersion::MQTT3, VersionCodec.decode(&mut buf).unwrap().unwrap());

        let mut buf =
            BytesMut::from(b"\x10\x98\x02\0\x04MQTT\x05\xc0\0\x0f\0\x02d1\0|testhub.".as_ref());
        assert_eq!(ProtocolVersion::MQTT5, VersionCodec.decode(&mut buf).unwrap().unwrap());

        let mut buf = BytesMut::from(b"\x10\x98\x02\0\x04MQTT\x05".as_ref());
        assert_eq!(ProtocolVersion::MQTT5, VersionCodec.decode(&mut buf).unwrap().unwrap());

        let mut buf = BytesMut::from(b"\x10\x98\x02\0\x04".as_ref());
        assert_eq!(None, VersionCodec.decode(&mut buf).unwrap());

        let mut buf = BytesMut::from(b"\x10\x98\x02\0\x04MQTT".as_ref());
        assert_eq!(None, VersionCodec.decode(&mut buf).unwrap());
    }
}
