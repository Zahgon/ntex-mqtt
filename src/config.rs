use ntex_service::cfg::{CfgContext, Configuration};
use ntex_util::time::{Millis, Seconds};

use crate::types::QoS;

#[derive(Debug)]
pub struct MqttServiceConfig {
    pub(crate) max_qos: QoS,
    pub(crate) max_size: u32,
    pub(crate) max_receive: u16,
    pub(crate) max_receive_size: usize,
    pub(crate) max_topic_alias: u16,
    pub(crate) max_send: u16,
    pub(crate) max_send_size: (u32, u32),
    pub(crate) min_chunk_size: u32,
    pub(crate) max_payload_buffer_size: usize,
    pub(crate) handle_qos_after_disconnect: Option<QoS>,
    pub(crate) connect_timeout: Seconds,
    pub(crate) handshake_timeout: Seconds,
    pub(crate) protocol_version_timeout: Millis,
    config: CfgContext,
}

impl Default for MqttServiceConfig {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl Configuration for MqttServiceConfig {
    const NAME: &str = "MQTT Service configuration";

    fn ctx(&self) -> &CfgContext { panic!("STUB: not implemented") }

    fn set_ctx(&mut self, ctx: CfgContext) { panic!("STUB: not implemented") }
}

impl MqttServiceConfig {
    pub fn new() -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn protocol_version_timeout(mut self, timeout: Seconds) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_connect_timeout(mut self, timeout: Seconds) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_max_qos(mut self, qos: QoS) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_max_size(mut self, size: u32) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_max_receive(mut self, val: u16) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_max_receive_size(mut self, val: usize) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_max_topic_alias(mut self, val: u16) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_max_send(mut self, val: u16) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_max_send_size(mut self, val: u32) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_min_chunk_size(mut self, size: u32) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_max_payload_buffer_size(mut self, val: usize) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_handle_qos_after_disconnect(mut self, max_handle_qos: Option<QoS>) -> Self { panic!("STUB: not implemented") }

    #[must_use]
    
    pub fn set_handshake_timeout(mut self, timeout: Seconds) -> Self { panic!("STUB: not implemented") }
}
