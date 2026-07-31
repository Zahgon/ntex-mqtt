use ntex_bytes::{Buf, BufMut, BytePages, ByteString, Bytes};

use crate::error::{DecodeError, EncodeError};
use crate::utils::{self, Decode, Property};
use crate::v5::codec::{UserProperties, UserProperty, encode, property_type as pt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Auth {
    pub reason_code: AuthReasonCode,
    pub auth_method: Option<ByteString>,
    pub auth_data: Option<Bytes>,
    pub reason_string: Option<ByteString>,
    pub user_properties: UserProperties,
}

prim_enum! {
    
    pub enum AuthReasonCode {
        Success = 0,
        ContinueAuth = 24,
        ReAuth = 25
    }
}

impl Auth {
    pub(crate) fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Default for Auth {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl encode::EncodeLtd for Auth {
    fn encoded_size(&self, limit: u32) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages, size: u32) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}
