use ntex_bytes::{Buf, BufMut, BytePages, ByteString, Bytes};

use crate::error::{DecodeError, EncodeError, ProtocolError};
use crate::utils::{self, Decode, Property};
use crate::v5::codec::{UserProperties, UserProperty, encode, property_type as pt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Disconnect {
    pub reason_code: DisconnectReasonCode,
    pub session_expiry_interval_secs: Option<u32>,
    pub server_reference: Option<ByteString>,
    pub reason_string: Option<ByteString>,
    pub user_properties: UserProperties,
}

prim_enum! {
    
    pub enum DisconnectReasonCode {
        NormalDisconnection = 0,
        DisconnectWithWillMessage = 4,
        UnspecifiedError = 128,
        MalformedPacket = 129,
        ProtocolError = 130,
        ImplementationSpecificError = 131,
        NotAuthorized = 135,
        ServerBusy = 137,
        ServerShuttingDown = 139,
        BadAuthenticationMethod = 140,
        KeepAliveTimeout = 141,
        SessionTakenOver = 142,
        TopicFilterInvalid = 143,
        TopicNameInvalid = 144,
        ReceiveMaximumExceeded = 147,
        TopicAliasInvalid = 148,
        PacketTooLarge = 149,
        MessageRateTooHigh = 150,
        QuotaExceeded = 151,
        AdministrativeAction = 152,
        PayloadFormatInvalid = 153,
        RetainNotSupported = 154,
        QosNotSupported = 155,
        UseAnotherServer = 156,
        ServerMoved = 157,
        SharedSubscriptionNotSupported = 158,
        ConnectionRateExceeded = 159,
        MaximumConnectTime = 160,
        SubscriptionIdentifiersNotSupported = 0xa1,
        WildcardSubscriptionsNotSupported = 162
    }
}

impl Disconnect {
    
    pub fn new(reason_code: DisconnectReasonCode) -> Self { panic!("STUB: not implemented") }

    pub fn from_proto_error(err: &ProtocolError) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn reason_string(mut self, reason: Option<ByteString>) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn server_reference(mut self, reference: ByteString) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn properties<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserProperties),
    { panic!("STUB: not implemented") }

    pub(crate) fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Default for Disconnect {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl encode::EncodeLtd for Disconnect {
    fn encoded_size(&self, limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}
