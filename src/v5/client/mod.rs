
use std::num::{NonZeroU16, NonZeroU32};

use ntex_bytes::{ByteString, Bytes};
use ntex_net::connect::Address;
use ntex_util::time::Seconds;

mod connection;
mod connector;
pub mod control;
mod dispatcher;

pub use self::connection::{Client, ClientRouter};
pub use self::connector::{MqttConnector, MqttConnectorService};
pub use self::control::{ProtocolMessage, ProtocolMessageAck};

pub use crate::topic::{TopicFilter, TopicFilterError};
pub use crate::types::QoS;
pub use crate::v5::{codec, error, sink::MqttSink};

#[derive(Clone, Debug)]
pub struct Connect<A: Address> {
    addr: A,
    pkt: codec::Connect,
}

impl<A: Address> Connect<A> {
    #[inline]
    
    pub fn new(addr: A) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn with(addr: A, pkt: codec::Connect) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn client_id<U>(mut self, client_id: U) -> Self
    where
        ByteString: From<U>,
    { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn clean_start(mut self) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn keep_alive(mut self, val: Seconds) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn last_will(mut self, val: codec::LastWill) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn auth(mut self, method: ByteString, data: Bytes) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn username(mut self, val: ByteString) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn password(mut self, val: Bytes) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn max_packet_size(mut self, val: u32) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn max_receive(mut self, val: u16) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn properties<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut codec::UserProperties),
    { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn packet<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut codec::Connect),
    { panic!("STUB: not implemented") }

    fn into_parts(self) -> (A, codec::Connect) { panic!("STUB: not implemented") }
}
