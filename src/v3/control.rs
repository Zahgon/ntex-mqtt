use std::{fmt, marker::PhantomData, num::NonZeroU16, ptr};

use ntex_bytes::ByteString;

use crate::{types::QoS, v3::codec};

#[derive(Debug)]
pub enum ProtocolMessage {
    
    PublishRelease(PublishRelease),
    
    Subscribe(Subscribe),
    
    Unsubscribe(Unsubscribe),
    
    Disconnect(Disconnect),
    
    Ping(Ping),
}

#[derive(Debug)]
pub struct ProtocolMessageAck {
    pub(crate) result: ProtocolMessageKind,
}

#[derive(Debug)]
pub(crate) enum ProtocolMessageKind {
    Nothing,
    PublishAck(NonZeroU16),
    PublishRelease(NonZeroU16),
    Ping,
    Disconnect,
    Subscribe(SubscribeResult),
    Unsubscribe(UnsubscribeResult),
}

impl ProtocolMessage {
    pub(crate) fn pubrel(packet_id: NonZeroU16) -> Self { panic!("STUB: not implemented") }

    pub fn ping() -> Self { panic!("STUB: not implemented") }

    pub fn subscribe(pkt: Subscribe) -> Self { panic!("STUB: not implemented") }

    pub fn unsubscribe(pkt: Unsubscribe) -> Self { panic!("STUB: not implemented") }

    pub fn remote_disconnect() -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn disconnect(&self) -> ProtocolMessageAck { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[derive(Copy, Clone, Debug)]
pub struct PublishRelease {
    pub packet_id: NonZeroU16,
}

impl PublishRelease {
    #[inline]
    
    pub fn id(self) -> NonZeroU16 { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[derive(Copy, Clone, Debug)]
pub struct Ping;

impl Ping {
    #[inline]
    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[derive(Copy, Clone, Debug)]
pub struct Disconnect;

impl Disconnect {
    #[inline]
    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
pub struct Subscribe {
    packet_id: NonZeroU16,
    packet_size: u32,
    topics: Vec<(ByteString, QoS)>,
    codes: Vec<codec::SubscribeReturnCode>,
}

#[derive(Debug, Clone)]
pub(crate) struct SubscribeResult {
    pub(crate) codes: Vec<codec::SubscribeReturnCode>,
    pub(crate) packet_id: NonZeroU16,
}

impl Subscribe {
    #[inline]
    
    pub fn new(
        packet_id: NonZeroU16,
        packet_size: u32,
        topics: Vec<(ByteString, QoS)>,
    ) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn iter_mut(&mut self) -> SubscribeIter<'_> { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

impl<'a> IntoIterator for &'a mut Subscribe {
    type Item = Subscription<'a>;
    type IntoIter = SubscribeIter<'a>;

    fn into_iter(self) -> SubscribeIter<'a> { panic!("STUB: not implemented") }
}

pub struct SubscribeIter<'a> {
    subs: *mut Subscribe,
    entry: usize,
    lt: PhantomData<&'a mut Subscribe>,
}

impl fmt::Debug for SubscribeIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<'a> SubscribeIter<'a> {
    fn next_unsafe(&mut self) -> Option<Subscription<'a>> { panic!("STUB: not implemented") }
}

impl<'a> Iterator for SubscribeIter<'a> {
    type Item = Subscription<'a>;

    #[inline]
    fn next(&mut self) -> Option<Subscription<'a>> { panic!("STUB: not implemented") }
}

#[derive(Debug)]
pub struct Subscription<'a> {
    topic: &'a ByteString,
    qos: QoS,
    code: &'a mut codec::SubscribeReturnCode,
}

impl<'a> Subscription<'a> {
    #[inline]
    
    pub fn topic(&self) -> &'a ByteString { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn qos(&self) -> QoS { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn fail(&mut self) { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn confirm(&mut self, qos: QoS) { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn subscribe(&mut self, qos: QoS) { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
pub struct Unsubscribe {
    packet_id: NonZeroU16,
    packet_size: u32,
    topics: Vec<ByteString>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct UnsubscribeResult {
    pub(crate) packet_id: NonZeroU16,
}

impl Unsubscribe {
    #[inline]
    
    pub fn new(packet_id: NonZeroU16, packet_size: u32, topics: Vec<ByteString>) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn iter(&self) -> impl Iterator<Item = &ByteString> {
        self.topics.iter()
    }

    #[inline]
    
    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;

    use ntex_bytes::ByteString;

    use super::*;
    use crate::types::QoS;

    #[test]
    fn test_debug() {
        
        let mut sub = Subscribe::new(
            NonZeroU16::new(1).unwrap(),
            0,
            vec![(ByteString::from_static("a/b"), QoS::AtLeastOnce)],
        );
        let iter = sub.iter_mut();
        assert!(format!("{iter:?}").contains("SubscribeIter"));

        let unsub = Unsubscribe::new(
            NonZeroU16::new(2).unwrap(),
            0,
            vec![ByteString::from_static("a/b")],
        );
        assert!(format!("{unsub:?}").contains("Unsubscribe"));

        assert!(format!("{Ping:?}").contains("Ping"));
        assert!(format!("{Disconnect:?}").contains("Disconnect"));
    }
}
