
use ntex_bytes::Bytes;

#[allow(clippy::module_inception)]
mod codec;
mod decode;
pub(crate) mod encode;
mod packet;

pub use self::codec::Codec;
pub use self::packet::{
    Connect, ConnectAck, ConnectAckReason, LastWill, Packet, Publish, SubscribeReturnCode,
};
pub use crate::types::{ConnectAckFlags, ConnectFlags, QoS};

#[derive(Clone, PartialEq, Eq)]
pub enum Decoded {
    Packet(Packet, u32),
    Publish(Publish, Bytes, u32),
    PayloadChunk(Bytes, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Encoded {
    Packet(Packet),
    Publish(Publish, Option<Bytes>),
    PayloadChunk(Bytes),
}

impl From<Packet> for Encoded {
    fn from(pkt: Packet) -> Encoded { panic!("STUB: not implemented") }
}

impl std::fmt::Debug for Decoded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { panic!("STUB: not implemented") }
}
