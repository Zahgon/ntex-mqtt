use ntex_bytes::{BufMut, BytePages, ByteString};

use crate::error::EncodeError;
use crate::types::{ConnectFlags, MQTT, MQTT_LEVEL_3, QoS, WILL_QOS_SHIFT, packet_type};
use crate::utils::{Encode, write_variable_length};

use super::packet::{Connect, LastWill, Packet, Publish, SubscribeReturnCode};

pub(crate) fn get_encoded_publish_size(p: &Publish) -> usize { panic!("STUB: not implemented") }

pub(crate) fn get_encoded_subscribe_size(topic_filters: &[(ByteString, QoS)]) -> usize { panic!("STUB: not implemented") }

pub(crate) fn get_encoded_unsubscribe_size(topic_filters: &[ByteString]) -> usize { panic!("STUB: not implemented") }

pub(crate) fn get_encoded_size(packet: &Packet) -> usize { panic!("STUB: not implemented") }

pub(crate) fn encode(
    packet: &Packet,
    dst: &mut BytePages,
    content_size: u32,
) -> Result<(), EncodeError> { panic!("STUB: not implemented") }

pub(super) fn encode_publish(
    publish: &Publish,
    dst: &mut BytePages,
    content_size: u32,
) -> Result<(), EncodeError> { panic!("STUB: not implemented") }

fn encode_connect(connect: &Connect, dst: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use ntex_bytes::Bytes;
    use std::num::NonZeroU16;

    use super::*;

    fn packet_id(v: u16) -> NonZeroU16 {
        NonZeroU16::new(v).unwrap()
    }

    #[test]
    fn test_encode_fixed_header() {
        let mut v = BytePages::default();
        let p = Packet::PingRequest;

        assert_eq!(get_encoded_size(&p), 0);
        encode(&p, &mut v, 0).unwrap();
        assert_eq!(v.freeze(), b"\xc0\x00".as_ref());

        let mut v = BytePages::default();

        let p = Publish {
            dup: true,
            retain: true,
            qos: QoS::ExactlyOnce,
            topic: ByteString::from_static("topic"),
            packet_id: Some(packet_id(0x4321)),
            payload_size: 255,
        };

        assert_eq!(get_encoded_publish_size(&p), 264);
        encode_publish(&p, &mut v, 264).unwrap();
        assert_eq!(&v.freeze()[0..3], b"\x3d\x88\x02".as_ref());
    }

    fn assert_encode_packet(packet: &Packet, expected: &[u8]) {
        let mut v = BytePages::default();
        encode(packet, &mut v, get_encoded_size(packet) as u32).unwrap();
        assert_eq!(expected.len(), v.len());
        assert_eq!(expected, &v.freeze()[..]);
    }

    fn assert_encode_publish(packet: &Publish, pl: &[u8], expected: &[u8]) {
        let mut v = BytePages::default();
        encode_publish(packet, &mut v, get_encoded_publish_size(packet) as u32).unwrap();
        v.extend_from_slice(pl);
        assert_eq!(expected.len(), v.len());
        assert_eq!(expected, &v.freeze()[..]);
    }

    #[test]
    fn test_encode_connect_packets() {
        assert_encode_packet(
            &Packet::Connect(Box::new(Connect {
                clean_session: false,
                keep_alive: 60,
                client_id: ByteString::from_static("12345"),
                last_will: None,
                username: Some(ByteString::from_static("user")),
                password: Some(Bytes::from_static(b"pass")),
            })),
            &b"\x10\x1D\x00\x04MQTT\x04\xC0\x00\x3C\x00\
\x0512345\x00\x04user\x00\x04pass"[..],
        );

        assert_encode_packet(
            &Packet::Connect(Box::new(Connect {
                clean_session: false,
                keep_alive: 60,
                client_id: ByteString::from_static("12345"),
                last_will: Some(LastWill {
                    qos: QoS::ExactlyOnce,
                    retain: false,
                    topic: ByteString::from_static("topic"),
                    message: Bytes::from_static(b"message"),
                }),
                username: None,
                password: None,
            })),
            &b"\x10\x21\x00\x04MQTT\x04\x14\x00\x3C\x00\
\x0512345\x00\x05topic\x00\x07message"[..],
        );

        assert_encode_packet(&Packet::Disconnect, b"\xe0\x00");
    }

    #[test]
    fn test_encode_publish_packets() {
        assert_encode_publish(
            &Publish {
                dup: true,
                retain: true,
                qos: QoS::ExactlyOnce,
                topic: ByteString::from_static("topic"),
                packet_id: Some(packet_id(0x4321)),
                payload_size: 4,
            },
            b"data",
            b"\x3d\x0D\x00\x05topic\x43\x21data",
        );

        assert_encode_publish(
            &Publish {
                dup: false,
                retain: false,
                qos: QoS::AtMostOnce,
                topic: ByteString::from_static("topic"),
                packet_id: None,
                payload_size: 4,
            },
            b"data",
            b"\x30\x0b\x00\x05topicdata",
        );
    }

    #[test]
    fn test_encode_subscribe_packets() {
        assert_encode_packet(
            &Packet::Subscribe {
                packet_id: packet_id(0x1234),
                topic_filters: vec![
                    (ByteString::from_static("test"), QoS::AtLeastOnce),
                    (ByteString::from_static("filter"), QoS::ExactlyOnce),
                ],
            },
            b"\x82\x12\x12\x34\x00\x04test\x01\x00\x06filter\x02",
        );

        assert_encode_packet(
            &Packet::SubscribeAck {
                packet_id: packet_id(0x1234),
                status: vec![
                    SubscribeReturnCode::Success(QoS::AtLeastOnce),
                    SubscribeReturnCode::Failure,
                    SubscribeReturnCode::Success(QoS::ExactlyOnce),
                ],
            },
            b"\x90\x05\x12\x34\x01\x80\x02",
        );

        assert_encode_packet(
            &Packet::Unsubscribe {
                packet_id: packet_id(0x1234),
                topic_filters: vec![
                    ByteString::from_static("test"),
                    ByteString::from_static("filter"),
                ],
            },
            b"\xa2\x10\x12\x34\x00\x04test\x00\x06filter",
        );

        assert_encode_packet(
            &Packet::UnsubscribeAck { packet_id: packet_id(0x4321) },
            b"\xb0\x02\x43\x21",
        );
    }

    #[test]
    fn test_encode_ping_packets() {
        assert_encode_packet(&Packet::PingRequest, b"\xc0\x00");
        assert_encode_packet(&Packet::PingResponse, b"\xd0\x00");
    }
}
