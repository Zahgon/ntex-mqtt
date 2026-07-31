use std::{fmt, num::NonZeroU32, rc::Rc};

use ntex_io::IoBoxed;
use ntex_util::time::Seconds;

use super::{codec as mqtt, shared::MqttShared, sink::MqttSink};

const DEFAULT_KEEPALIVE: Seconds = Seconds(30);

pub struct Handshake {
    io: IoBoxed,
    pkt: Box<mqtt::Connect>,
    pkt_size: u32,
    shared: Rc<MqttShared>,
}

impl Handshake {
    pub(crate) fn new(
        pkt: Box<mqtt::Connect>,
        pkt_size: u32,
        io: IoBoxed,
        shared: Rc<MqttShared>,
    ) -> Self { panic!("STUB: not implemented") }

    #[inline]
    pub fn packet(&self) -> &mqtt::Connect { panic!("STUB: not implemented") }

    #[inline]
    pub fn packet_mut(&mut self) -> &mut mqtt::Connect { panic!("STUB: not implemented") }

    #[inline]
    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }

    #[inline]
    pub fn io(&self) -> &IoBoxed { panic!("STUB: not implemented") }

    pub fn sink(&self) -> MqttSink { panic!("STUB: not implemented") }

    pub fn ack<St>(self, st: St, session_present: bool) -> HandshakeAck<St> { panic!("STUB: not implemented") }

    pub fn identifier_rejected<St>(self) -> HandshakeAck<St> { panic!("STUB: not implemented") }

    pub fn bad_username_or_pwd<St>(self) -> HandshakeAck<St> { panic!("STUB: not implemented") }

    pub fn not_authorized<St>(self) -> HandshakeAck<St> { panic!("STUB: not implemented") }

    pub fn service_unavailable<St>(self) -> HandshakeAck<St> { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn failed<St>(self, return_code: mqtt::ConnectAckReason) -> HandshakeAck<St> { panic!("STUB: not implemented") }
}

impl fmt::Debug for Handshake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct HandshakeAck<St> {
    pub(crate) io: IoBoxed,
    pub(crate) session: Option<St>,
    pub(crate) session_present: bool,
    pub(crate) return_code: mqtt::ConnectAckReason,
    pub(crate) shared: Rc<MqttShared>,
    pub(crate) keepalive: Seconds,
    pub(crate) max_send: Option<u16>,
    pub(crate) max_packet_size: Option<NonZeroU32>,
}

impl<St> fmt::Debug for HandshakeAck<St> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<St> HandshakeAck<St> {
    #[must_use]
    
    pub fn idle_timeout(mut self, timeout: Seconds) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn max_send(mut self, val: Option<u16>) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn max_packet_size(mut self, val: NonZeroU32) -> Self { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use ntex_io::{Io, IoBoxed, testing::IoTest};
    use ntex_service::cfg::SharedCfg;

    use super::*;
    use crate::v3::shared::MqttShared;

    #[ntex::test]
    async fn test_debug() {
        let io = Io::new(IoTest::create().0, SharedCfg::new("test"));
        let codec = mqtt::Codec::default();
        let shared = Rc::new(MqttShared::new(io.get_ref(), codec, false, Rc::default()));
        let connect = Box::new(mqtt::Connect::default());
        let h = Handshake::new(connect, 0, IoBoxed::from(io), shared);

        let dbg = format!("{h:?}");
        assert!(!dbg.is_empty());

        let ack = h.ack(42u32, false);
        let dbg = format!("{ack:?}");
        assert!(dbg.contains("HandshakeAck"));
        assert!(dbg.contains("session_present"));
    }
}
