use std::num::{NonZeroU16, NonZeroU32};

use ntex_bytes::{Buf, BufMut, BytePages, ByteString, Bytes};

use crate::error::{DecodeError, EncodeError};
use crate::types::{ConnectFlags, MQTT, MQTT_LEVEL_5, QoS, WILL_QOS_SHIFT};
use crate::utils::{self, Decode, Encode, Property};
use crate::v5::codec::{UserProperties, UserProperty, encode, property_type as pt};

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Connect {
    
    pub clean_start: bool,
    
    pub keep_alive: u16,

    pub session_expiry_interval_secs: u32,
    pub auth_method: Option<ByteString>,
    pub auth_data: Option<Bytes>,
    pub request_problem_info: bool,
    pub request_response_info: bool,
    pub receive_max: Option<NonZeroU16>,
    pub topic_alias_max: u16,
    pub user_properties: UserProperties,
    pub max_packet_size: Option<NonZeroU32>,

    pub last_will: Option<LastWill>,
    
    pub client_id: ByteString,
    
    pub username: Option<ByteString>,
    
    pub password: Option<Bytes>,
}

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct LastWill {
    
    pub qos: QoS,
    
    pub retain: bool,
    
    pub topic: ByteString,
    
    pub message: Bytes,

    pub will_delay_interval_sec: Option<u32>,
    pub correlation_data: Option<Bytes>,
    pub message_expiry_interval: Option<NonZeroU32>,
    pub content_type: Option<ByteString>,
    pub user_properties: UserProperties,
    pub is_utf8_payload: Option<bool>,
    pub response_topic: Option<ByteString>,
}

impl LastWill {
    fn properties_len(&self) -> usize { panic!("STUB: not implemented") }
}

impl Connect {
    #[must_use]
    
    pub fn client_id<T>(mut self, client_id: T) -> Self
    where
        ByteString: From<T>,
    { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn receive_max(mut self, max: u16) -> Self { panic!("STUB: not implemented") }

    fn properties_len(&self) -> usize { panic!("STUB: not implemented") }

    pub(crate) fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Default for Connect {
    fn default() -> Connect { panic!("STUB: not implemented") }
}

fn decode_last_will(src: &mut Bytes, flags: ConnectFlags) -> Result<LastWill, DecodeError> { panic!("STUB: not implemented") }

impl encode::EncodeLtd for Connect {
    fn encoded_size(&self, _limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, _size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}
