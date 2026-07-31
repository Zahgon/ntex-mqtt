use std::{cell::Cell, fmt, mem};

use ntex_bytes::{Bytes, BytesMut};
use ntex_util::{channel::bstream, future::Either};

pub(crate) use ntex_util::channel::bstream::Status as PayloadStatus;

use crate::error::PayloadError;

type PlStream = bstream::Receiver<PayloadError>;
pub(crate) type PlSender = bstream::Sender<PayloadError>;

pub struct Payload {
    pl: Either<Cell<Option<Bytes>>, PlStream>,
}

impl Default for Payload {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl Payload {
    pub fn from_bytes(buf: Bytes) -> Payload { panic!("STUB: not implemented") }

    pub(crate) fn from_stream(buf: Bytes, buf_size: usize) -> (Payload, PlSender) { panic!("STUB: not implemented") }

    pub fn is_fixed(&self) -> bool { panic!("STUB: not implemented") }

    pub async fn read(&self) -> Result<Option<Bytes>, PayloadError> { panic!("STUB: not implemented") }

    pub async fn read_all(&self) -> Result<Bytes, PayloadError> { panic!("STUB: not implemented") }

    #[must_use]
    pub fn take(&mut self) -> Payload { panic!("STUB: not implemented") }
}

impl fmt::Debug for Payload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ntex::test]
    async fn test_payload() {
        let (pl, tx) = Payload::from_stream(Bytes::from(b"chunk1"), 32 * 1024);

        ntex::rt::spawn(async move {
            ntex::time::sleep(ntex::time::Millis(50)).await;
            tx.feed_data(b"chunk2".into());
            ntex::time::sleep(ntex::time::Millis(50)).await;
            tx.feed_data(b"chunk3".into());
            tx.feed_eof();
        });

        let data = pl.read_all().await.unwrap();
        assert_eq!(data, b"chunk1chunk2chunk3");
    }
}
