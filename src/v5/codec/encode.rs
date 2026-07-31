#![allow(clippy::ref_option, clippy::needless_pass_by_value)]
use ntex_bytes::{BufMut, BytePages, ByteString};

use super::packet::{Packet, property_type as pt};
use super::{UserProperties, UserProperty};
use crate::error::EncodeError;
use crate::types::packet_type;
use crate::utils::{Encode, write_variable_length};

pub(crate) trait EncodeLtd {
    fn encoded_size(&self, limit: u32) -> usize;

    fn encode(&self, buf: &mut BytePages, size: u32) -> Result<(), EncodeError>;
}

impl EncodeLtd for Packet {
    fn encoded_size(&self, limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, check_size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

pub(crate) fn encoded_size_opt_props(
    user_props: &[UserProperty],
    reason_str: &Option<ByteString>,
    mut limit: u32,
) -> usize { panic!("STUB: not implemented") }

pub(crate) fn encode_opt_props(
    user_props: &[UserProperty],
    reason_str: &Option<ByteString>,
    buf: &mut BytePages,
    mut size: u32,
) -> Result<(), EncodeError> { panic!("STUB: not implemented") }

pub(super) fn encoded_property_size<T: Encode>(v: &Option<T>) -> usize { panic!("STUB: not implemented") }

pub(super) fn encoded_property_size_default<T: Encode + PartialEq>(v: &T, default: T) -> usize { panic!("STUB: not implemented") }

pub(super) fn encode_property<T: Encode>(
    v: &Option<T>,
    prop_type: u8,
    buf: &mut BytePages,
) -> Result<(), EncodeError> { panic!("STUB: not implemented") }

pub(super) fn encode_property_default<T: Encode + PartialEq>(
    v: &T,
    default: T,
    prop_type: u8,
    buf: &mut BytePages,
) -> Result<(), EncodeError> { panic!("STUB: not implemented") }

pub(crate) fn var_int_len(val: usize) -> u32 { panic!("STUB: not implemented") }

pub(crate) fn var_int_len_u32(val: u32) -> u32 { panic!("STUB: not implemented") }

pub(crate) fn var_int_len_from_size(val: u32) -> u32 { panic!("STUB: not implemented") }

impl Encode for UserProperties {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }
    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

pub(super) fn reduce_limit(limit: u32, reduction: usize) -> u32 { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use ntex_bytes::Bytes;
    use std::num::{NonZeroU16, NonZeroU32};

    use super::*;
    use crate::types::{MAX_PACKET_SIZE, QoS};
    use crate::v5::codec::*;

    fn packet_id(v: u16) -> NonZeroU16 {
        NonZeroU16::new(v).unwrap()
    }

    #[test]
    fn test_encode_fixed_header() {
        let mut v = BytePages::default();
        let p = Packet::PingRequest;

        assert_eq!(p.encoded_size(MAX_PACKET_SIZE), 0);
        p.encode(&mut v, 0).unwrap();
        assert_eq!(&v.freeze()[..2], b"\xc0\x00".as_ref());

        let mut v = BytePages::default();

        let p = Publish {
            dup: true,
            retain: true,
            qos: QoS::ExactlyOnce,
            topic: ByteString::from_static("topic"),
            packet_id: Some(packet_id(0x4321)),
            payload_size: 255,
            properties: PublishProperties::default(),
        };

        p.encode(&mut v, 265).unwrap();
        assert_eq!(&v.freeze()[..3], b"\x3d\x89\x02".as_ref());
    }

    fn assert_encode_packet(packet: &Packet, expected: &[u8]) {
        let mut v = BytePages::default();
        packet.encode(&mut v, packet.encoded_size(1024) as u32).unwrap();
        assert_eq!(expected.len(), v.len());
        assert_eq!(expected, &v.freeze()[..]);
    }

    fn assert_encode_publish(packet: &Publish, pl: &[u8], expected: &[u8]) {
        let mut v = BytePages::default();
        packet.encode(&mut v, packet.encoded_size(1024) as u32).unwrap();
        v.extend_from_slice(pl);
        assert_eq!(expected.len(), v.len());
        assert_eq!(expected, &v.freeze()[..]);
    }

    #[test]
    fn test_encode_connect_packets() {
        assert_encode_packet(
            &Packet::Connect(Box::new(Connect {
                clean_start: false,
                keep_alive: 60,
                client_id: ByteString::from_static("12345"),
                last_will: None,
                username: Some(ByteString::from_static("user")),
                password: Some(Bytes::from_static(b"pass")),
                session_expiry_interval_secs: 0,
                auth_method: None,
                auth_data: None,
                request_problem_info: true,
                request_response_info: false,
                receive_max: None,
                topic_alias_max: 0,
                user_properties: vec![],
                max_packet_size: None,
            })),
            &b"\x10\x1E\x00\x04MQTT\x05\xC0\x00\x3C\x00\x00\
\x0512345\x00\x04user\x00\x04pass"[..],
        );

        assert_encode_packet(
            &Packet::Connect(Box::new(Connect {
                clean_start: false,
                keep_alive: 60,
                client_id: ByteString::from_static("12345"),
                last_will: Some(LastWill {
                    qos: QoS::ExactlyOnce,
                    retain: false,
                    topic: ByteString::from_static("topic"),
                    message: Bytes::from_static(b"message"),
                    will_delay_interval_sec: None,
                    correlation_data: None,
                    message_expiry_interval: None,
                    content_type: None,
                    user_properties: vec![],
                    is_utf8_payload: None,
                    response_topic: None,
                }),
                username: None,
                password: None,
                session_expiry_interval_secs: 0,
                auth_method: None,
                auth_data: None,
                request_problem_info: true,
                request_response_info: false,
                receive_max: None,
                topic_alias_max: 0,
                user_properties: vec![],
                max_packet_size: None,
            })),
            &b"\x10\x23\x00\x04MQTT\x05\x14\x00\x3C\x00\x00\
\x0512345\x00\x00\x05topic\x00\x07message"[..],
        );

        assert_encode_packet(
            &Packet::Connect(Box::new(Connect {
                clean_start: false,
                keep_alive: 60,
                client_id: ByteString::from_static("12345"),
                last_will: Some(LastWill {
                    qos: QoS::ExactlyOnce,
                    retain: true,
                    topic: ByteString::from_static("topic"),
                    message: Bytes::from_static(b"message"),
                    will_delay_interval_sec: Some(5),
                    correlation_data: Some(Bytes::from_static(b"correlationData")),
                    message_expiry_interval: NonZeroU32::new(7),
                    content_type: Some(ByteString::from_static("contentType")),
                    user_properties: vec![
                        (ByteString::from_static("name"), ByteString::from_static("value"))
                    ],
                    is_utf8_payload: Some(true),
                    response_topic: Some(ByteString::from_static("responseTopic")),
                }),
                username: None,
                password: None,
                session_expiry_interval_secs: 0,
                auth_method: None,
                auth_data: None,
                request_problem_info: true,
                request_response_info: false,
                receive_max: None,
                topic_alias_max: 0,
                user_properties: vec![],
                max_packet_size: None,
            })),
            &b"\x10\x6D\x00\x04MQTT\x05\x34\x00\x3C\x00\x00\
\x0512345\x4A\x18\0\0\0\x05\x01\x01\x02\0\0\0\x07\x03\0\x0bcontentType\x08\x00\x0dresponseTopic\x09\0\x0fcorrelationData\x26\0\x04name\0\x05value\x00\x05topic\x00\x07message"[..],
        );

        assert_encode_packet(
            &Packet::Disconnect(Disconnect {
                reason_code: DisconnectReasonCode::NormalDisconnection,
                session_expiry_interval_secs: None,
                server_reference: None,
                reason_string: None,
                user_properties: vec![],
            }),
            b"\xe0\x02\x00\x00",
        );
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
                properties: PublishProperties::default(),
            },
            b"data",
            b"\x3d\x0E\x00\x05topic\x43\x21\x00data",
        );

        assert_encode_publish(
            &Publish {
                dup: false,
                retain: false,
                qos: QoS::AtMostOnce,
                topic: ByteString::from_static("topic"),
                packet_id: None,
                payload_size: 4,
                properties: PublishProperties::default(),
            },
            b"data",
            b"\x30\x0c\x00\x05topic\x00data",
        );

        assert_encode_publish(
            &Publish {
                dup: false,
                retain: false,
                qos: QoS::AtMostOnce,
                topic: ByteString::from_static("topic"),
                packet_id: None,
                payload_size: 4,
                properties: PublishProperties {
                    subscription_ids: vec![NonZeroU32::new(1).unwrap()],
                    ..Default::default()
                },
            },
            b"data",
            b"\x30\x0e\x00\x05topic\x02\x0b\x01data",
        );
    }

    #[test]
    fn test_encode_subscribe_packets() {
        assert_encode_packet(
            &Packet::Subscribe(Subscribe {
                packet_id: packet_id(0x1234),
                id: None,
                user_properties: Vec::new(),
                topic_filters: vec![
                    (
                        ByteString::from_static("test"),
                        SubscriptionOptions {
                            qos: QoS::AtLeastOnce,
                            no_local: false,
                            retain_as_published: false,
                            retain_handling: RetainHandling::AtSubscribe,
                        },
                    ),
                    (
                        ByteString::from_static("filter"),
                        SubscriptionOptions {
                            qos: QoS::ExactlyOnce,
                            no_local: false,
                            retain_as_published: false,
                            retain_handling: RetainHandling::AtSubscribe,
                        },
                    ),
                ],
            }),
            b"\x82\x13\x12\x34\x00\x00\x04test\x01\x00\x06filter\x02",
        );

        assert_encode_packet(
            &Packet::Subscribe(Subscribe {
                packet_id: packet_id(0x1234),
                id: Some(NonZeroU32::new(1).unwrap()),
                user_properties: Vec::new(),
                topic_filters: vec![
                    (
                        ByteString::from_static("test"),
                        SubscriptionOptions {
                            qos: QoS::AtLeastOnce,
                            no_local: false,
                            retain_as_published: false,
                            retain_handling: RetainHandling::AtSubscribe,
                        },
                    ),
                    (
                        ByteString::from_static("filter"),
                        SubscriptionOptions {
                            qos: QoS::ExactlyOnce,
                            no_local: false,
                            retain_as_published: false,
                            retain_handling: RetainHandling::AtSubscribe,
                        },
                    ),
                ],
            }),
            b"\x82\x15\x12\x34\x02\x0b\x01\x00\x04test\x01\x00\x06filter\x02",
        );

        assert_encode_packet(
            &Packet::SubscribeAck(SubscribeAck {
                packet_id: packet_id(0x1234),
                properties: UserProperties::default(),
                reason_string: None,
                status: vec![
                    SubscribeAckReason::GrantedQos1,
                    SubscribeAckReason::UnspecifiedError,
                    SubscribeAckReason::GrantedQos2,
                ],
            }),
            b"\x90\x06\x12\x34\x00\x01\x80\x02",
        );

        assert_encode_packet(
            &Packet::Unsubscribe(Unsubscribe {
                packet_id: packet_id(0x1234),
                topic_filters: vec![
                    ByteString::from_static("test"),
                    ByteString::from_static("filter"),
                ],
                user_properties: Vec::new(),
            }),
            b"\xa2\x11\x12\x34\x00\x00\x04test\x00\x06filter",
        );

        assert_encode_packet(
            &Packet::UnsubscribeAck(UnsubscribeAck {
                packet_id: packet_id(0x4321),
                properties: UserProperties::default(),
                reason_string: None,
                status: vec![
                    UnsubscribeAckReason::Success,
                    UnsubscribeAckReason::NotAuthorized,
                ],
            }),
            b"\xb0\x05\x43\x21\x00\x00\x87",
        );
    }

    #[test]
    fn test_encode_ping_packets() {
        assert_encode_packet(&Packet::PingRequest, b"\xc0\x00");
        assert_encode_packet(&Packet::PingResponse, b"\xd0\x00");
    }
}
