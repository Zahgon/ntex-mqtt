
use ntex_bytes::{ByteString, Bytes};

#[allow(clippy::module_inception)]
mod codec;
mod decode;
mod encode;
mod packet;

pub use self::codec::Codec;
pub(crate) use self::encode::EncodeLtd;
pub use self::packet::*;

pub type UserProperty = (ByteString, ByteString);
pub type UserProperties = Vec<UserProperty>;

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
