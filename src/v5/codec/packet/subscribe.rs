use std::num::{NonZeroU16, NonZeroU32};

use ntex_bytes::{Buf, BufMut, BytePages, ByteString, Bytes};

use super::ack_props;
use crate::error::{DecodeError, EncodeError};
use crate::types::QoS;
use crate::utils::{self, Decode, Encode, write_variable_length};
use crate::v5::codec::{UserProperties, UserProperty, encode, property_type as pt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Subscribe {
    
    pub packet_id: NonZeroU16,
    
    pub id: Option<NonZeroU32>,
    pub user_properties: UserProperties,
    
    pub topic_filters: Vec<(ByteString, SubscriptionOptions)>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct SubscriptionOptions {
    pub qos: QoS,
    pub no_local: bool,
    pub retain_as_published: bool,
    pub retain_handling: RetainHandling,
}

impl Default for SubscriptionOptions {
    fn default() -> Self { panic!("STUB: not implemented") }
}

prim_enum! {
    pub enum RetainHandling {
        AtSubscribe = 0,
        AtSubscribeNew = 1,
        NoAtSubscribe = 2
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubscribeAck {
    pub packet_id: NonZeroU16,
    pub properties: UserProperties,
    pub reason_string: Option<ByteString>,
    
    pub status: Vec<SubscribeAckReason>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Unsubscribe {
    
    pub packet_id: NonZeroU16,
    pub user_properties: UserProperties,
    
    pub topic_filters: Vec<ByteString>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnsubscribeAck {
    
    pub packet_id: NonZeroU16,
    pub properties: UserProperties,
    pub reason_string: Option<ByteString>,
    pub status: Vec<UnsubscribeAckReason>,
}

prim_enum! {
    
    pub enum SubscribeAckReason {
        GrantedQos0 = 0,
        GrantedQos1 = 1,
        GrantedQos2 = 2,
        UnspecifiedError = 128,
        ImplementationSpecificError = 131,
        NotAuthorized = 135,
        TopicFilterInvalid = 143,
        PacketIdentifierInUse = 145,
        QuotaExceeded = 151,
        SharedSubscriptionNotSupported = 158,
        SubscriptionIdentifiersNotSupported = 161,
        WildcardSubscriptionsNotSupported = 162
    }
}

prim_enum! {
    
    pub enum UnsubscribeAckReason {
        Success = 0,
        NoSubscriptionExisted = 17,
        UnspecifiedError = 128,
        ImplementationSpecificError = 131,
        NotAuthorized = 135,
        TopicFilterInvalid = 143,
        PacketIdentifierInUse = 145
    }
}

impl Subscribe {
    pub(crate) fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl SubscribeAck {
    pub(crate) fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Unsubscribe {
    pub(crate) fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl UnsubscribeAck {
    pub(crate) fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl encode::EncodeLtd for Subscribe {
    fn encoded_size(&self, _limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, _: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Decode for SubscriptionOptions {
    fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Encode for SubscriptionOptions {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl encode::EncodeLtd for SubscribeAck {
    fn encoded_size(&self, limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl encode::EncodeLtd for Unsubscribe {
    fn encoded_size(&self, _limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, _size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl encode::EncodeLtd for UnsubscribeAck {
    
    fn encoded_size(&self, limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use ntex_codec::{Decoder, Encoder};

    use super::super::super::{Codec, Decoded, EncodeLtd, Packet};
    
    use super::*;

    fn packet(res: Decoded) -> Packet {
        match res {
            Decoded::Packet(pkt, _) => pkt,
            _ => panic!(),
        }
    }

    #[test]
    fn test_sub() {
        let pkt = Subscribe {
            packet_id: 12.try_into().unwrap(),
            id: Some(10.try_into().unwrap()),
            user_properties: vec![("a".into(), "1".into())],
            topic_filters: vec![("test".into(), SubscriptionOptions::default())],
        };

        let size = pkt.encoded_size(99999);
        let mut buf = BytePages::default();
        pkt.encode(&mut buf, size as u32).unwrap();
        assert_eq!(buf.len(), size);
        assert_eq!(pkt, Subscribe::decode(&mut buf.take().unwrap().freeze()).unwrap());

        let pkt = Unsubscribe {
            packet_id: 12.try_into().unwrap(),
            user_properties: vec![("a".into(), "1".into())],
            topic_filters: vec!["test".into()],
        };

        let size = pkt.encoded_size(99999);
        let mut buf = BytePages::default();
        pkt.encode(&mut buf, size as u32).unwrap();
        assert_eq!(buf.len(), size);
        assert_eq!(pkt, Unsubscribe::decode(&mut buf.take().unwrap().freeze()).unwrap());
    }

    #[test]
    fn test_sub_pkt() {
        let pkt = Packet::Subscribe(Subscribe {
            packet_id: 12.try_into().unwrap(),
            id: None,
            user_properties: vec![("a".into(), "1".into())],
            topic_filters: vec![("test".into(), SubscriptionOptions::default())],
        });
        let codec = Codec::new();

        let mut buf = BytePages::default();
        codec.encodev(pkt.clone().into(), &mut buf).unwrap();

        assert_eq!(
            pkt,
            packet(codec.decode(&mut buf.take().unwrap().into()).unwrap().unwrap())
        );
    }

    #[test]
    fn test_sub_ack() {
        let ack = SubscribeAck {
            packet_id: NonZeroU16::new(1).unwrap(),
            properties: Vec::new(),
            reason_string: Some("some reason".into()),
            status: Vec::new(),
        };

        let size = ack.encoded_size(99999);
        let mut buf = BytePages::default();
        ack.encode(&mut buf, size as u32).unwrap();
        assert_eq!(ack, SubscribeAck::decode(&mut buf.take().unwrap().freeze()).unwrap());

        let ack = SubscribeAck {
            packet_id: NonZeroU16::new(1).unwrap(),
            properties: vec![("prop1".into(), "val1".into()), ("prop2".into(), "val2".into())],
            reason_string: None,
            status: vec![SubscribeAckReason::GrantedQos0],
        };
        let size = ack.encoded_size(99999);
        let mut buf = BytePages::default();
        ack.encode(&mut buf, size as u32).unwrap();
        assert_eq!(ack, SubscribeAck::decode(&mut buf.take().unwrap().freeze()).unwrap());

        let ack = UnsubscribeAck {
            packet_id: NonZeroU16::new(1).unwrap(),
            properties: Vec::new(),
            reason_string: Some("some reason".into()),
            status: Vec::new(),
        };
        let mut buf = BytePages::default();
        let size = ack.encoded_size(99999);
        ack.encode(&mut buf, size as u32).unwrap();
        assert_eq!(ack, UnsubscribeAck::decode(&mut buf.take().unwrap().freeze()).unwrap());

        let ack = UnsubscribeAck {
            packet_id: NonZeroU16::new(1).unwrap(),
            properties: vec![("prop1".into(), "val1".into()), ("prop2".into(), "val2".into())],
            reason_string: None,
            status: vec![UnsubscribeAckReason::Success],
        };
        let size = ack.encoded_size(99999);
        let mut buf = BytePages::default();
        ack.encode(&mut buf, size as u32).unwrap();
        assert_eq!(ack, UnsubscribeAck::decode(&mut buf.take().unwrap().freeze()).unwrap());
    }
}
