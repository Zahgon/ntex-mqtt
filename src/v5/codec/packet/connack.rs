use std::num::NonZeroU16;

use ntex_bytes::{Buf, BufMut, BytePages, ByteString, Bytes};

use crate::error::{DecodeError, EncodeError};
use crate::types::{ConnectAckFlags, QoS};
use crate::utils::{self, Decode, Encode, Property};
use crate::v5::RECEIVE_MAX_DEFAULT;
use crate::v5::codec::{UserProperties, UserProperty, encode, property_type as pt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConnectAck {
    
    pub session_present: bool,
    pub reason_code: ConnectAckReason,

    pub session_expiry_interval_secs: Option<u32>,
    pub receive_max: NonZeroU16,
    pub max_qos: QoS,
    pub max_packet_size: Option<u32>,
    pub assigned_client_id: Option<ByteString>,
    pub topic_alias_max: u16,
    pub retain_available: bool,
    pub wildcard_subscription_available: bool,
    pub subscription_identifiers_available: bool,
    pub shared_subscription_available: bool,
    pub server_keepalive_sec: Option<u16>,
    pub response_info: Option<ByteString>,
    pub server_reference: Option<ByteString>,
    pub auth_method: Option<ByteString>,
    pub auth_data: Option<Bytes>,
    pub reason_string: Option<ByteString>,
    pub user_properties: UserProperties,
}

impl Default for ConnectAck {
    fn default() -> ConnectAck { panic!("STUB: not implemented") }
}

prim_enum! {
    
    pub enum ConnectAckReason {
        Success = 0,
        UnspecifiedError = 128,
        MalformedPacket = 129,
        ProtocolError = 130,
        ImplementationSpecificError = 131,
        UnsupportedProtocolVersion = 132,
        ClientIdentifierNotValid = 133,
        BadUserNameOrPassword = 134,
        NotAuthorized = 135,
        ServerUnavailable = 136,
        ServerBusy = 137,
        Banned = 138,
        BadAuthenticationMethod = 140,
        TopicNameInvalid = 144,
        PacketTooLarge = 149,
        QuotaExceeded = 151,
        PayloadFormatInvalid = 153,
        RetainNotSupported = 154,
        QosNotSupported = 155,
        UseAnotherServer = 156,
        ServerMoved = 157,
        ConnectionRateExceeded = 159
    }
}

impl ConnectAckReason {
    pub fn reason(self) -> &'static str { panic!("STUB: not implemented") }
}

impl ConnectAck {
    pub(crate) fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl encode::EncodeLtd for ConnectAck {
    fn encoded_size(&self, limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}
