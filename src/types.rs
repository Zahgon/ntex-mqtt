pub(crate) const MQTT: &[u8] = b"MQTT";
pub(crate) const MQTT_LEVEL_3: u8 = 4;
pub(crate) const MQTT_LEVEL_5: u8 = 5;
pub(crate) const WILL_QOS_SHIFT: u8 = 3;

pub(crate) const MAX_PACKET_SIZE: u32 = 0xF_FF_FF_FF;

prim_enum! {
    
    #[derive(serde::Serialize, serde::Deserialize, PartialOrd, Ord, Hash)]
    pub enum QoS {
        
        AtMostOnce = 0,
        
        AtLeastOnce = 1,
        
        ExactlyOnce = 2
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ConnectFlags: u8 {
        const USERNAME    = 0b1000_0000;
        const PASSWORD    = 0b0100_0000;
        const WILL_RETAIN = 0b0010_0000;
        const WILL_QOS    = 0b0001_1000;
        const WILL        = 0b0000_0100;
        const CLEAN_START = 0b0000_0010;
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ConnectAckFlags: u8 {
        const SESSION_PRESENT = 0b0000_0001;
    }
}

pub(super) mod packet_type {
    pub(crate) const CONNECT: u8 = 0b0001_0000;
    pub(crate) const CONNACK: u8 = 0b0010_0000;
    pub(crate) const PUBLISH_START: u8 = 0b0011_0000;
    pub(crate) const PUBLISH_END: u8 = 0b0011_1111;
    pub(crate) const PUBACK: u8 = 0b0100_0000;
    pub(crate) const PUBREC: u8 = 0b0101_0000;
    pub(crate) const PUBREL: u8 = 0b0110_0010;
    pub(crate) const PUBCOMP: u8 = 0b0111_0000;
    pub(crate) const SUBSCRIBE: u8 = 0b1000_0010;
    pub(crate) const SUBACK: u8 = 0b1001_0000;
    pub(crate) const UNSUBSCRIBE: u8 = 0b1010_0010;
    pub(crate) const UNSUBACK: u8 = 0b1011_0000;
    pub(crate) const PINGREQ: u8 = 0b1100_0000;
    pub(crate) const PINGRESP: u8 = 0b1101_0000;
    pub(crate) const DISCONNECT: u8 = 0b1110_0000;
    pub(crate) const AUTH: u8 = 0b1111_0000;

    pub(crate) const fn is_publish(packet_type: u8) -> bool {
        matches!(packet_type, PUBLISH_START..=PUBLISH_END)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct FixedHeader {
    
    pub(crate) first_byte: u8,
    
    pub(crate) remaining_length: u32,
}
