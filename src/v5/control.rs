use std::{fmt, marker::PhantomData, ptr};

use ntex_bytes::ByteString;

use crate::{error, types::QoS, v5::codec};

#[derive(Debug)]
pub enum ProtocolMessage {
    
    Auth(Auth),
    
    PublishRelease(PublishRelease),
    
    Subscribe(Subscribe),
    
    Unsubscribe(Unsubscribe),
    
    Disconnect(Disconnect),
    
    Ping(Ping),
}

#[derive(Debug)]
pub(crate) enum Pkt {
    None,
    Disconnect(codec::Disconnect),
    Packet(codec::Packet),
}

#[derive(Debug)]
pub struct ProtocolMessageAck {
    pub(crate) packet: Pkt,
    pub(crate) disconnect: bool,
}

impl ProtocolMessage {
    
    pub fn auth(pkt: codec::Auth, size: u32) -> Self { panic!("STUB: not implemented") }

    pub(crate) fn pubrel(pkt: codec::PublishAck2, size: u32) -> Self { panic!("STUB: not implemented") }

    pub fn subscribe(pkt: codec::Subscribe, size: u32) -> Self { panic!("STUB: not implemented") }

    pub fn unsubscribe(pkt: codec::Unsubscribe, size: u32) -> Self { panic!("STUB: not implemented") }

    pub fn ping() -> Self { panic!("STUB: not implemented") }

    pub fn remote_disconnect(pkt: codec::Disconnect, size: u32) -> Self { panic!("STUB: not implemented") }

    pub fn disconnect(&self) -> ProtocolMessageAck { panic!("STUB: not implemented") }

    pub fn disconnect_with(&self, pkt: codec::Disconnect) -> ProtocolMessageAck { panic!("STUB: not implemented") }

    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[derive(Debug)]
pub struct Auth {
    pkt: codec::Auth,
    size: u32,
}

impl Auth {
    
    pub fn packet(&self) -> &codec::Auth { panic!("STUB: not implemented") }

    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn ack(self, response: codec::Auth) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
pub struct PublishRelease {
    pkt: codec::PublishAck2,
    result: codec::PublishAck2,
    size: u32,
}

impl PublishRelease {
    pub(crate) fn new(pkt: codec::PublishAck2, size: u32) -> Self { panic!("STUB: not implemented") }

    pub fn packet(&self) -> &codec::PublishAck2 { panic!("STUB: not implemented") }

    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn properties<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut codec::UserProperties),
    { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn reason(mut self, reason: ByteString) -> Self { panic!("STUB: not implemented") }

    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[derive(Debug, Copy, Clone)]
pub struct Ping;

impl Ping {
    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
pub struct Disconnect(pub(crate) codec::Disconnect, pub(crate) u32);

impl Disconnect {
    
    pub fn packet(&self) -> &codec::Disconnect { panic!("STUB: not implemented") }

    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
pub struct Subscribe {
    packet: codec::Subscribe,
    result: codec::SubscribeAck,
    size: u32,
}

impl Subscribe {
    
    pub fn new(packet: codec::Subscribe, size: u32) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn iter_mut(&mut self) -> SubscribeIter<'_> { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn ack_reason(mut self, reason: ByteString) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn ack_properties<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut codec::UserProperties),
    { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet(&self) -> &codec::Subscribe { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }
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
    options: &'a codec::SubscriptionOptions,
    status: &'a mut codec::SubscribeAckReason,
}

impl<'a> Subscription<'a> {
    #[inline]
    
    pub fn topic(&self) -> &'a ByteString { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn options(&self) -> &codec::SubscriptionOptions { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn fail(&mut self, status: codec::SubscribeAckReason) { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn confirm(&mut self, qos: QoS) { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn subscribe(&mut self, qos: QoS) { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
pub struct Unsubscribe {
    packet: codec::Unsubscribe,
    result: codec::UnsubscribeAck,
    size: u32,
}

impl Unsubscribe {
    
    pub fn new(packet: codec::Unsubscribe, size: u32) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn properties(&self) -> &codec::UserProperties { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn iter(&self) -> impl Iterator<Item = &ByteString> {
        self.packet.topic_filters.iter()
    }

    #[inline]
    
    pub fn iter_mut(&mut self) -> UnsubscribeIter<'_> { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn ack_reason(mut self, reason: ByteString) -> Self { panic!("STUB: not implemented") }

    #[inline]
    #[must_use]
    
    pub fn ack_properties<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut codec::UserProperties),
    { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn ack(self) -> ProtocolMessageAck { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet(&self) -> &codec::Unsubscribe { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn packet_size(&self) -> u32 { panic!("STUB: not implemented") }
}

impl<'a> IntoIterator for &'a mut Unsubscribe {
    type Item = UnsubscribeItem<'a>;
    type IntoIter = UnsubscribeIter<'a>;

    fn into_iter(self) -> UnsubscribeIter<'a> { panic!("STUB: not implemented") }
}

pub struct UnsubscribeIter<'a> {
    subs: *mut Unsubscribe,
    entry: usize,
    lt: PhantomData<&'a mut Unsubscribe>,
}

impl fmt::Debug for UnsubscribeIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<'a> UnsubscribeIter<'a> {
    fn next_unsafe(&mut self) -> Option<UnsubscribeItem<'a>> { panic!("STUB: not implemented") }
}

impl<'a> Iterator for UnsubscribeIter<'a> {
    type Item = UnsubscribeItem<'a>;

    #[inline]
    fn next(&mut self) -> Option<UnsubscribeItem<'a>> { panic!("STUB: not implemented") }
}

#[derive(Debug)]
pub struct UnsubscribeItem<'a> {
    topic: &'a ByteString,
    status: &'a mut codec::UnsubscribeAckReason,
}

impl<'a> UnsubscribeItem<'a> {
    #[inline]
    
    pub fn topic(&self) -> &'a ByteString { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn fail(&mut self, status: codec::UnsubscribeAckReason) { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn success(&mut self) { panic!("STUB: not implemented") }
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
            codec::Subscribe {
                packet_id: NonZeroU16::new(1).unwrap(),
                id: None,
                user_properties: Vec::new(),
                topic_filters: vec![(
                    ByteString::from_static("a/b"),
                    codec::SubscriptionOptions {
                        qos: QoS::AtLeastOnce,
                        no_local: false,
                        retain_as_published: false,
                        retain_handling: codec::RetainHandling::AtSubscribe,
                    },
                )],
            },
            0,
        );
        let iter = sub.iter_mut();
        assert!(format!("{iter:?}").contains("SubscribeIter"));

        let mut unsub = Unsubscribe::new(
            codec::Unsubscribe {
                packet_id: NonZeroU16::new(2).unwrap(),
                user_properties: Vec::new(),
                topic_filters: vec![ByteString::from_static("a/b")],
            },
            0,
        );
        let uiter = unsub.iter_mut();
        assert!(format!("{uiter:?}").contains("UnsubscribeIter"));

        assert!(format!("{Ping:?}").contains("Ping"));
    }
}
