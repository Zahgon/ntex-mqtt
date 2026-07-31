use std::{io::Cursor, num::NonZeroU16, num::NonZeroU32};

use ntex_bytes::{Buf, BufMut, BytePages, ByteString, Bytes};

use crate::error::{DecodeError, EncodeError};

macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            return Err($e);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)+) => {
        if !($cond) {
            return Err($fmt, $($arg)+);
        }
    };
}

macro_rules! prim_enum {
    (
        $( #[$enum_attr:meta] )*
        pub enum $name:ident {
            $(
                $( #[$enum_item_attr:meta] )*
                $var:ident=$val:expr
            ),+
        }) => {
        $( #[$enum_attr] )*
        #[repr(u8)]
        #[derive(Debug, Eq, PartialEq, Copy, Clone)]
        pub enum $name {
            $(
                $( #[$enum_item_attr] )*
                $var = $val
            ),+
        }
        impl std::convert::TryFrom<u8> for $name {
            type Error = $crate::error::DecodeError;
            fn try_from(v: u8) -> Result<Self, Self::Error> {
                match v {
                    $($val => Ok($name::$var)),+
                    ,_ => Err($crate::error::DecodeError::MalformedPacket)
                }
            }
        }
        impl From<$name> for u8 {
            fn from(v: $name) -> Self {
                unsafe { ::std::mem::transmute(v) }
            }
        }
    };
}

pub(crate) trait Decode: Sized {
    fn decode(src: &mut Bytes) -> Result<Self, DecodeError>;
}

pub(super) trait Property {
    fn read_value(&mut self, src: &mut Bytes) -> Result<(), DecodeError>;
}

impl<T: Decode> Property for Option<T> {
    fn read_value(&mut self, src: &mut Bytes) -> Result<(), DecodeError> { panic!("STUB: not implemented") }
}

impl Decode for bool {
    fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Decode for u16 {
    fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Decode for u32 {
    fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Decode for NonZeroU32 {
    fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Decode for NonZeroU16 {
    fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Decode for Bytes {
    fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

impl Decode for ByteString {
    fn decode(src: &mut Bytes) -> Result<Self, DecodeError> { panic!("STUB: not implemented") }
}

pub(crate) fn take_properties(src: &mut Bytes) -> Result<Bytes, DecodeError> { panic!("STUB: not implemented") }

pub(crate) fn decode_variable_length(src: &[u8]) -> Result<Option<(u32, usize)>, DecodeError> { panic!("STUB: not implemented") }

#[allow(clippy::cast_lossless)] 
pub(crate) fn decode_variable_length_cursor<B: Buf>(src: &mut B) -> Result<u32, DecodeError> { panic!("STUB: not implemented") }

pub(crate) trait Encode {
    fn encoded_size(&self) -> usize;

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError>;
}

impl<T: Encode> Encode for Option<T> {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Encode for bool {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Encode for u16 {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Encode for NonZeroU16 {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Encode for u32 {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Encode for NonZeroU32 {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Encode for Bytes {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Encode for ByteString {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Encode for (ByteString, ByteString) {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

impl Encode for &[u8] {
    fn encoded_size(&self) -> usize { panic!("STUB: not implemented") }

    fn encode(&self, buf: &mut BytePages) -> Result<(), EncodeError> { panic!("STUB: not implemented") }
}

pub(crate) fn write_variable_length(len: u32, dst: &mut BytePages) { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_variable_length() {
        fn assert_variable_length<B: AsRef<[u8]> + 'static>(bytes: B, res: (u32, usize)) {
            assert_eq!(decode_variable_length(bytes.as_ref()), Ok(Some(res)));
        }

        assert_variable_length(b"\x7f\x7f", (127, 1));

        assert_eq!(decode_variable_length(b"\xff\xff\xff"), Ok(None));

        assert_eq!(
            decode_variable_length(b"\xff\xff\xff\xff\xff\xff"),
            Err(DecodeError::InvalidLength)
        );

        assert_variable_length(b"\x00", (0, 1));
        assert_variable_length(b"\x7f", (127, 1));
        assert_variable_length(b"\x80\x01", (128, 2));
        assert_variable_length(b"\xff\x7f", (16383, 2));
        assert_variable_length(b"\x80\x80\x01", (16384, 3));
        assert_variable_length(b"\xff\xff\x7f", (2_097_151, 3));
        assert_variable_length(b"\x80\x80\x80\x01", (2_097_152, 4));
        assert_variable_length(b"\xff\xff\xff\x7f", (268_435_455, 4));
    }

    #[test]
    fn test_encode_variable_length() {
        let mut v = BytePages::default();

        write_variable_length(123, &mut v);
        assert_eq!(v.take().unwrap().freeze(), [123].as_ref());

        write_variable_length(129, &mut v);
        assert_eq!(v.take().unwrap().freeze(), b"\x81\x01".as_ref());

        write_variable_length(16_383, &mut v);
        assert_eq!(v.take().unwrap().freeze(), b"\xff\x7f".as_ref());

        write_variable_length(2_097_151, &mut v);
        assert_eq!(v.take().unwrap().freeze(), b"\xff\xff\x7f".as_ref());

        write_variable_length(268_435_455, &mut v);
        assert_eq!(v.take().unwrap().freeze(), b"\xff\xff\xff\x7f".as_ref());

    }
}
