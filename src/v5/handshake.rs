use ntex_io::IoBoxed;
use std::{fmt, num::NonZeroU16, rc::Rc};

use super::{codec, shared::MqttShared, sink::MqttSink};

pub struct Handshake {
    io: IoBoxed,
    pkt: Box<codec::Connect>,
    size: u32,
    pub(super) shared: Rc<MqttShared>,
}

impl Handshake {
    pub(crate) fn new(
        pkt: Box<codec::Connect>,
        size: u32,
        io: IoBoxed,
        shared: Rc<MqttShared>,
    ) -> Self { panic!("STUB: not implemented") }

    #[inline]
    pub fn packet(&self) -> &codec::Connect { panic!("STUB: not implemented") }

    #[inline]
    pub fn packet_mut(&mut self) -> &mut codec::Connect { panic!("STUB: not implemented") }

    #[inline]
    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }

    #[inline]
    pub fn io(&self) -> &IoBoxed { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn sink(&self) -> MqttSink { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn ack<St>(self, st: St) -> HandshakeAck<St> { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn failed<St>(self, reason_code: codec::ConnectAckReason) -> HandshakeAck<St> { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn fail_with<St>(self, ack: codec::ConnectAck) -> HandshakeAck<St> { panic!("STUB: not implemented") }
}

impl fmt::Debug for Handshake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct HandshakeAck<St> {
    pub(crate) io: IoBoxed,
    pub(crate) session: Option<St>,
    pub(crate) shared: Rc<MqttShared>,
    pub(crate) packet: codec::ConnectAck,
    pub(crate) keepalive: u16,
    pub(crate) max_send: Option<u16>,
}

impl<St> fmt::Debug for HandshakeAck<St> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<St> HandshakeAck<St> {
    #[inline]
    #[must_use]
    
    pub fn keep_alive(mut self, timeout: u16) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn max_send(mut self, val: Option<u16>) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn with(mut self, f: impl FnOnce(&mut codec::ConnectAck)) -> Self { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use ntex_io::{Io, IoBoxed, testing::IoTest};
    use ntex_service::cfg::SharedCfg;

    use super::*;
    use crate::v5::shared::MqttShared;

    #[ntex::test]
    async fn test_debug() {
        let io = Io::new(IoTest::create().0, SharedCfg::new("test"));
        let codec_v5 = codec::Codec::new();
        let shared = Rc::new(MqttShared::new(io.get_ref(), codec_v5, Rc::default()));
        let connect = Box::new(codec::Connect::default());
        let h = Handshake::new(connect, 0, IoBoxed::from(io), shared);

        let dbg = format!("{h:?}");
        assert!(!dbg.is_empty());

        let ack = h.ack(42u32);
        let dbg = format!("{ack:?}");
        assert!(dbg.contains("HandshakeAck"));
        assert!(dbg.contains("keepalive"));
    }
}
