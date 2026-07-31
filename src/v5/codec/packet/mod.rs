#![allow(clippy::struct_excessive_bools)]
use ntex_bytes::{Buf, BufMut, BytePages, ByteString, Bytes};

pub use crate::types::{ConnectAckFlags, ConnectFlags, QoS};

use super::{UserProperties, encode, property_type as pt};
use crate::error::{DecodeError, EncodeError};
use crate::types::packet_type;
use crate::utils::{Decode, Property, take_properties, write_variable_length};

mod auth;
mod connack;
mod connect;
mod disconnect;
mod pubacks;
mod publish;
mod subscribe;

pub use auth::*;
pub use connack::*;
pub use connect::*;
pub use disconnect::*;
pub use pubacks::*;
pub use publish::*;
pub use subscribe::*;

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Packet {
    
    Connect(Box<Connect>),
    
    ConnectAck(Box<ConnectAck>),
    
    PublishAck(PublishAck),
    
    PublishReceived(PublishAck),
    
    PublishRelease(PublishAck2),
    
    PublishComplete(PublishAck2),
    
    Subscribe(Subscribe),
    
    SubscribeAck(SubscribeAck),
    
    Unsubscribe(Unsubscribe),
    
    UnsubscribeAck(UnsubscribeAck),
    
    PingRequest,
    
    PingResponse,
    
    Disconnect(Disconnect),
    
    Auth(Auth),
}

impl Packet {
    pub fn packet_type(&self) -> u8 { panic!("STUB: not implemented") }
}

impl From<Connect> for Packet {
    fn from(pkt: Connect) -> Self { panic!("STUB: not implemented") }
}

impl From<Box<Connect>> for Packet {
    fn from(pkt: Box<Connect>) -> Self { panic!("STUB: not implemented") }
}

impl From<ConnectAck> for Packet {
    fn from(pkt: ConnectAck) -> Self { panic!("STUB: not implemented") }
}

impl From<Box<ConnectAck>> for Packet {
    fn from(pkt: Box<ConnectAck>) -> Self { panic!("STUB: not implemented") }
}

impl From<PublishAck> for Packet {
    fn from(pkt: PublishAck) -> Self { panic!("STUB: not implemented") }
}

impl From<Subscribe> for Packet {
    fn from(pkt: Subscribe) -> Self { panic!("STUB: not implemented") }
}

impl From<SubscribeAck> for Packet {
    fn from(pkt: SubscribeAck) -> Self { panic!("STUB: not implemented") }
}

impl From<Unsubscribe> for Packet {
    fn from(pkt: Unsubscribe) -> Self { panic!("STUB: not implemented") }
}

impl From<UnsubscribeAck> for Packet {
    fn from(pkt: UnsubscribeAck) -> Self { panic!("STUB: not implemented") }
}

impl From<Disconnect> for Packet {
    fn from(pkt: Disconnect) -> Self { panic!("STUB: not implemented") }
}

impl From<Auth> for Packet {
    fn from(pkt: Auth) -> Self { panic!("STUB: not implemented") }
}

pub(super) mod property_type {
    pub(crate) const UTF8_PAYLOAD: u8 = 0x01;
    pub(crate) const MSG_EXPIRY_INT: u8 = 0x02;
    pub(crate) const CONTENT_TYPE: u8 = 0x03;
    pub(crate) const RESP_TOPIC: u8 = 0x08;
    pub(crate) const CORR_DATA: u8 = 0x09;
    pub(crate) const SUB_ID: u8 = 0x0B;
    pub(crate) const SESS_EXPIRY_INT: u8 = 0x11;
    pub(crate) const ASSND_CLIENT_ID: u8 = 0x12;
    pub(crate) const SERVER_KA: u8 = 0x13;
    pub(crate) const AUTH_METHOD: u8 = 0x15;
    pub(crate) const AUTH_DATA: u8 = 0x16;
    pub(crate) const REQ_PROB_INFO: u8 = 0x17;
    pub(crate) const WILL_DELAY_INT: u8 = 0x18;
    pub(crate) const REQ_RESP_INFO: u8 = 0x19;
    pub(crate) const RESP_INFO: u8 = 0x1A;
    pub(crate) const SERVER_REF: u8 = 0x1C;
    pub(crate) const REASON_STRING: u8 = 0x1F;
    pub(crate) const RECEIVE_MAX: u8 = 0x21;
    pub(crate) const TOPIC_ALIAS_MAX: u8 = 0x22;
    pub(crate) const TOPIC_ALIAS: u8 = 0x23;
    pub(crate) const MAX_QOS: u8 = 0x24;
    pub(crate) const RETAIN_AVAIL: u8 = 0x25;
    pub(crate) const USER: u8 = 0x26;
    pub(crate) const MAX_PACKET_SIZE: u8 = 0x27;
    pub(crate) const WILDCARD_SUB_AVAIL: u8 = 0x28;
    pub(crate) const SUB_IDS_AVAIL: u8 = 0x29;
    pub(crate) const SHARED_SUB_AVAIL: u8 = 0x2A;
}

#[allow(clippy::ref_option, clippy::wildcard_imports)]
mod ack_props {
    use super::*;
    use crate::v5::codec::UserProperty;

    pub(crate) fn encoded_size(
        properties: &[UserProperty],
        reason_string: &Option<ByteString>,
        limit: u32,
    ) -> usize { panic!("STUB: not implemented") }

    pub(crate) fn encode(
        properties: &[UserProperty],
        reason_string: &Option<ByteString>,
        buf: &mut BytePages,
        size: u32,
    ) -> Result<(), EncodeError> { panic!("STUB: not implemented") }

    pub(crate) fn decode(
        src: &mut Bytes,
    ) -> Result<(UserProperties, Option<ByteString>), DecodeError> { panic!("STUB: not implemented") }
}
