use std::num::NonZeroU16;

use ntex_bytes::{ByteString, Bytes};

use crate::types::{QoS, packet_type};

prim_enum! {
    
    pub enum ConnectAckReason {
        
        ConnectionAccepted = 0,
        
        UnacceptableProtocolVersion = 1,
        
        IdentifierRejected = 2,
        
        ServiceUnavailable = 3,
        
        BadUserNameOrPassword = 4,
        
        NotAuthorized = 5,
        
        Reserved = 6
    }
}

impl ConnectAckReason {
    pub fn reason(self) -> &'static str { panic!("STUB: not implemented") }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct LastWill {
    
    pub qos: QoS,
    
    pub retain: bool,
    
    pub topic: ByteString,
    
    pub message: Bytes,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]

pub struct Connect {
    
    pub clean_session: bool,
    
    pub keep_alive: u16,
    
    pub last_will: Option<LastWill>,
    
    pub client_id: ByteString,
    
    pub username: Option<ByteString>,
    
    pub password: Option<Bytes>,
}

impl Connect {
    #[must_use]
    
    pub fn client_id<T>(mut self, client_id: T) -> Self
    where
        ByteString: From<T>,
    { panic!("STUB: not implemented") }
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Publish {
    
    pub dup: bool,
    pub retain: bool,
    
    pub qos: QoS,
    
    pub topic: ByteString,
    
    pub packet_id: Option<NonZeroU16>,
    
    pub payload_size: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]

pub struct ConnectAck {
    pub return_code: ConnectAckReason,
    
    pub session_present: bool,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]

pub enum SubscribeReturnCode {
    Success(QoS),
    Failure,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub enum Packet {
    
    Connect(Box<Connect>),
    
    ConnectAck(ConnectAck),
    
    PublishAck {
        
        packet_id: NonZeroU16,
    },
    
    PublishReceived {
        
        packet_id: NonZeroU16,
    },
    
    PublishRelease {
        
        packet_id: NonZeroU16,
    },
    
    PublishComplete {
        
        packet_id: NonZeroU16,
    },
    
    Subscribe {
        
        packet_id: NonZeroU16,
        
        topic_filters: Vec<(ByteString, QoS)>,
    },
    
    SubscribeAck {
        packet_id: NonZeroU16,
        
        status: Vec<SubscribeReturnCode>,
    },
    
    Unsubscribe {
        
        packet_id: NonZeroU16,
        
        topic_filters: Vec<ByteString>,
    },
    
    UnsubscribeAck {
        
        packet_id: NonZeroU16,
    },
    
    PingRequest,
    
    PingResponse,
    
    Disconnect,
}

impl From<Connect> for Packet {
    fn from(val: Connect) -> Packet { panic!("STUB: not implemented") }
}

impl Packet {
    pub fn packet_type(&self) -> u8 { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ack_reason() {
        assert_eq!(ConnectAckReason::ConnectionAccepted.reason(), "Connection Accepted");
        assert_eq!(
            ConnectAckReason::UnacceptableProtocolVersion.reason(),
            "Connection Refused, unacceptable protocol version"
        );
        assert_eq!(
            ConnectAckReason::IdentifierRejected.reason(),
            "Connection Refused, identifier rejected"
        );
        assert_eq!(
            ConnectAckReason::ServiceUnavailable.reason(),
            "Connection Refused, Server unavailable"
        );
        assert_eq!(
            ConnectAckReason::BadUserNameOrPassword.reason(),
            "Connection Refused, bad user name or password"
        );
        assert_eq!(
            ConnectAckReason::NotAuthorized.reason(),
            "Connection Refused, not authorized"
        );
    }
}
