use std::num::NonZeroU16;

use ntex_bytes::{Buf, ByteString, Bytes, BytesMut};

use crate::error::DecodeError;
use crate::types::{MQTT, MQTT_LEVEL_3, QoS, WILL_QOS_SHIFT, packet_type};
use crate::utils::Decode;

use super::packet::{Connect, ConnectAck, LastWill, Packet, Publish, SubscribeReturnCode};
use super::{ConnectAckFlags, ConnectFlags};

pub(crate) fn decode_packet(mut src: Bytes, first_byte: u8) -> Result<Packet, DecodeError> { panic!("STUB: not implemented") }

#[inline]
fn decode_ack(mut src: Bytes, f: impl Fn(NonZeroU16) -> Packet) -> Result<Packet, DecodeError> { panic!("STUB: not implemented") }

fn decode_connect_packet(src: &mut Bytes) -> Result<Packet, DecodeError> { panic!("STUB: not implemented") }

fn decode_connect_ack_packet(src: &mut Bytes) -> Result<Packet, DecodeError> { panic!("STUB: not implemented") }

pub(super) fn decode_publish_packet(
    src: &mut Bytes,
    packet_flags: u8,
    payload_size: u32,
) -> Result<Publish, DecodeError> { panic!("STUB: not implemented") }

pub(super) fn publish_size(src: &BytesMut, flags: u8) -> Result<Option<u32>, DecodeError> { panic!("STUB: not implemented") }

fn decode_subscribe_packet(src: &mut Bytes) -> Result<Packet, DecodeError> { panic!("STUB: not implemented") }

fn decode_subscribe_ack_packet(src: &mut Bytes) -> Result<Packet, DecodeError> { panic!("STUB: not implemented") }

fn decode_unsubscribe_packet(src: &mut Bytes) -> Result<Packet, DecodeError> { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::decode_variable_length;
    use crate::v3::codec::ConnectAckReason;

    macro_rules! assert_decode_packet (
        ($bytes:expr, $res:expr) => {{
            let first_byte = $bytes.as_ref()[0];
            let (_len, consumed) = decode_variable_length(&$bytes[1..]).unwrap().unwrap();
            let cur = Bytes::from_static(&$bytes[consumed + 1..]);
            assert_eq!(decode_packet(cur, first_byte), Ok($res));
        }};
    );

    macro_rules! assert_decode_publish (
        ($bytes:expr, $res:expr, $pl:expr) => {{
            let first_byte = $bytes.as_ref()[0];
            let (_len, consumed) = decode_variable_length(&$bytes[1..]).unwrap().unwrap();
            let mut cur = Bytes::from_static(&$bytes[consumed + 1..]);
            assert_eq!(decode_publish_packet(&mut cur, first_byte, $pl.len() as u32), Ok($res));
            assert_eq!(cur, $pl);
        }};
    );

    fn packet_id(v: u16) -> NonZeroU16 {
        NonZeroU16::new(v).unwrap()
    }

    #[test]
    fn test_decode_connect_packets() {
        assert_eq!(
            decode_connect_packet(&mut Bytes::from_static(
                b"\x00\x04MQTT\x04\xC0\x00\x3C\x00\x0512345\x00\x04user\x00\x04pass"
            )),
            Ok(Packet::Connect(Box::new(Connect {
                clean_session: false,
                keep_alive: 60,
                client_id: ByteString::try_from(Bytes::from_static(b"12345")).unwrap(),
                last_will: None,
                username: Some(ByteString::try_from(Bytes::from_static(b"user")).unwrap()),
                password: Some(Bytes::from(&b"pass"[..])),
            })))
        );

        assert_eq!(
            decode_connect_packet(&mut Bytes::from_static(
                b"\x00\x04MQTT\x04\x14\x00\x3C\x00\x0512345\x00\x05topic\x00\x07message"
            )),
            Ok(Packet::Connect(Box::new(Connect {
                clean_session: false,
                keep_alive: 60,
                client_id: ByteString::try_from(Bytes::from_static(b"12345")).unwrap(),
                last_will: Some(LastWill {
                    qos: QoS::ExactlyOnce,
                    retain: false,
                    topic: ByteString::try_from(Bytes::from_static(b"topic")).unwrap(),
                    message: Bytes::from(&b"message"[..]),
                }),
                username: None,
                password: None,
            })))
        );

        assert_eq!(
            decode_connect_packet(&mut Bytes::from_static(b"\x00\x02MQ00000000000000000000")),
            Err(DecodeError::InvalidProtocol),
        );
        assert_eq!(
            decode_connect_packet(&mut Bytes::from_static(b"\x00\x10MQ00000000000000000000")),
            Err(DecodeError::InvalidProtocol),
        );
        assert_eq!(
            decode_connect_packet(&mut Bytes::from_static(b"\x00\x04MQAA00000000000000000000")),
            Err(DecodeError::InvalidProtocol),
        );
        assert_eq!(
            decode_connect_packet(&mut Bytes::from_static(
                b"\x00\x04MQTT\x0300000000000000000000"
            )),
            Err(DecodeError::UnsupportedProtocolLevel),
        );
        assert_eq!(
            decode_connect_packet(&mut Bytes::from_static(
                b"\x00\x04MQTT\x04\xff00000000000000000000"
            )),
            Err(DecodeError::ConnectReservedFlagSet)
        );

        assert_eq!(
            decode_connect_ack_packet(&mut Bytes::from_static(b"\x01\x04")),
            Ok(Packet::ConnectAck(ConnectAck {
                session_present: true,
                return_code: ConnectAckReason::BadUserNameOrPassword
            }))
        );

        assert_eq!(
            decode_connect_ack_packet(&mut Bytes::from_static(b"\x03\x04")),
            Err(DecodeError::ConnAckReservedFlagSet)
        );

        assert_decode_packet!(
            b"\x20\x02\x01\x04",
            Packet::ConnectAck(ConnectAck {
                session_present: true,
                return_code: ConnectAckReason::BadUserNameOrPassword,
            })
        );

        assert_decode_packet!(b"\xe0\x00", Packet::Disconnect);
    }

    #[test]
    fn test_decode_publish_packets() {
        
        assert_decode_publish!(
            b"\x3d\x0D\x00\x05topic\x43\x21data",
            Publish {
                dup: true,
                retain: true,
                qos: QoS::ExactlyOnce,
                topic: ByteString::try_from(Bytes::from_static(b"topic")).unwrap(),
                packet_id: Some(packet_id(0x4321)),
                payload_size: 4,
            },
            Bytes::from_static(b"data")
        );
        assert_decode_publish!(
            b"\x30\x0b\x00\x05topicdata",
            Publish {
                dup: false,
                retain: false,
                qos: QoS::AtMostOnce,
                topic: ByteString::try_from(Bytes::from_static(b"topic")).unwrap(),
                packet_id: None,
                payload_size: 4,
            },
            Bytes::from_static(b"data")
        );

        assert_decode_packet!(
            b"\x40\x02\x43\x21",
            Packet::PublishAck { packet_id: packet_id(0x4321) }
        );
        assert_decode_packet!(
            b"\x50\x02\x43\x21",
            Packet::PublishReceived { packet_id: packet_id(0x4321) }
        );
        assert_decode_packet!(
            b"\x62\x02\x43\x21",
            Packet::PublishRelease { packet_id: packet_id(0x4321) }
        );
        assert_decode_packet!(
            b"\x70\x02\x43\x21",
            Packet::PublishComplete { packet_id: packet_id(0x4321) }
        );
    }

    #[test]
    fn test_decode_subscribe_packets() {
        let p = Packet::Subscribe {
            packet_id: packet_id(0x1234),
            topic_filters: vec![
                (ByteString::try_from(Bytes::from_static(b"test")).unwrap(), QoS::AtLeastOnce),
                (
                    ByteString::try_from(Bytes::from_static(b"filter")).unwrap(),
                    QoS::ExactlyOnce,
                ),
            ],
        };

        assert_eq!(
            decode_subscribe_packet(&mut Bytes::from_static(
                b"\x12\x34\x00\x04test\x01\x00\x06filter\x02"
            )),
            Ok(p.clone())
        );
        assert_decode_packet!(b"\x82\x12\x12\x34\x00\x04test\x01\x00\x06filter\x02", p);

        let p = Packet::SubscribeAck {
            packet_id: packet_id(0x1234),
            status: vec![
                SubscribeReturnCode::Success(QoS::AtLeastOnce),
                SubscribeReturnCode::Failure,
                SubscribeReturnCode::Success(QoS::ExactlyOnce),
            ],
        };

        assert_eq!(
            decode_subscribe_ack_packet(&mut Bytes::from_static(b"\x12\x34\x01\x80\x02")),
            Ok(p.clone())
        );
        assert_decode_packet!(b"\x90\x05\x12\x34\x01\x80\x02", p);

        let p = Packet::Unsubscribe {
            packet_id: packet_id(0x1234),
            topic_filters: vec![
                ByteString::try_from(Bytes::from_static(b"test")).unwrap(),
                ByteString::try_from(Bytes::from_static(b"filter")).unwrap(),
            ],
        };

        assert_eq!(
            decode_unsubscribe_packet(&mut Bytes::from_static(
                b"\x12\x34\x00\x04test\x00\x06filter"
            )),
            Ok(p.clone())
        );
        assert_decode_packet!(b"\xa2\x10\x12\x34\x00\x04test\x00\x06filter", p);

        assert_decode_packet!(
            b"\xb0\x02\x43\x21",
            Packet::UnsubscribeAck { packet_id: packet_id(0x4321) }
        );
    }

    #[test]
    fn test_decode_ping_packets() {
        assert_decode_packet!(b"\xc0\x00", Packet::PingRequest);
        assert_decode_packet!(b"\xd0\x00", Packet::PingResponse);
    }
}
