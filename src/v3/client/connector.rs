use std::{fmt, marker::PhantomData, rc::Rc};

use ntex_io::IoBoxed;
use ntex_net::connect::{self, Address, Connector};
use ntex_service::cfg::{Cfg, SharedCfg};
use ntex_service::{IntoServiceFactory, Service, ServiceCtx, ServiceFactory};
use ntex_util::time::{Seconds, timeout_checked};

use super::{Connect, connection::Client, error::ClientError, error::ProtocolError};
use crate::MqttServiceConfig;
use crate::v3::codec::{self, Decoded, Encoded};
use crate::v3::shared::{MqttShared, MqttSinkPool};

pub struct MqttConnector<A, T> {
    connector: T,
    pool: Rc<MqttSinkPool>,
    _t: PhantomData<A>,
}

impl<A, T> fmt::Debug for MqttConnector<A, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct MqttConnectorService<A, T> {
    connector: T,
    cfg: Cfg<MqttServiceConfig>,
    pool: Rc<MqttSinkPool>,
    _t: PhantomData<A>,
}

impl<A, T> fmt::Debug for MqttConnectorService<A, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<A> MqttConnector<A, ()>
where
    A: Address,
{
    #[allow(clippy::new_ret_no_self)]
    
    pub fn new() -> MqttConnector<A, Connector<A>> { panic!("STUB: not implemented") }
}

impl<A, T> MqttConnector<A, T>
where
    A: Address,
{
    
    pub fn connector<U, F>(self, connector: F) -> MqttConnector<A, U>
    where
        F: IntoServiceFactory<U, connect::Connect<A>, SharedCfg>,
        U: ServiceFactory<connect::Connect<A>, SharedCfg, Error = connect::ConnectError>,
        IoBoxed: From<U::Response>,
    { panic!("STUB: not implemented") }
}

impl<A, T> ServiceFactory<Connect<A>, SharedCfg> for MqttConnector<A, T>
where
    A: Address,
    T: ServiceFactory<connect::Connect<A>, SharedCfg, Error = connect::ConnectError>,
    IoBoxed: From<T::Response>,
{
    type Response = Client;
    type Error = ClientError<codec::ConnectAck>;
    type Service = MqttConnectorService<A, T::Service>;
    type InitError = T::InitError;

    async fn create(&self, cfg: SharedCfg) -> Result<Self::Service, Self::InitError> { panic!("STUB: not implemented") }
}

impl<A, T> Service<Connect<A>> for MqttConnectorService<A, T>
where
    A: Address,
    T: Service<connect::Connect<A>, Error = connect::ConnectError>,
    IoBoxed: From<T::Response>,
{
    type Response = Client;
    type Error = ClientError<codec::ConnectAck>;

    ntex_service::forward_ready!(connector);
    ntex_service::forward_poll!(connector);
    ntex_service::forward_shutdown!(connector);

    async fn call(
        &self,
        req: Connect<A>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Client, Self::Error> { panic!("STUB: not implemented") }
}

impl<A, T> MqttConnectorService<A, T>
where
    A: Address,
    T: Service<connect::Connect<A>, Error = connect::ConnectError>,
    IoBoxed: From<T::Response>,
{
    async fn connect_inner(
        &self,
        addr: A,
        pkt: codec::Connect,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Client, ClientError<codec::ConnectAck>> { panic!("STUB: not implemented") }
}
