
use std::{io, marker::PhantomData};

use ntex_service::{Service, ServiceCtx, ServiceFactory};

use crate::error;

#[derive(Debug)]
pub enum Control<E> {
    
    WrBackpressure(WrBackpressure),
    
    Stop(Reason<E>),
}

#[derive(Debug)]
pub enum Reason<E> {
    
    Error(Error<E>),
    
    Protocol(ProtocolError),
    
    PeerGone(PeerGone),
}

impl<E> Control<E> {
    pub(super) fn wr(state: bool) -> Self { panic!("STUB: not implemented") }

    pub(super) fn err(err: E) -> Self { panic!("STUB: not implemented") }

    pub(super) fn peer_gone(err: Option<io::Error>) -> Self { panic!("STUB: not implemented") }

    pub(super) fn proto(err: error::ProtocolError) -> Self { panic!("STUB: not implemented") }
}

#[derive(Debug, Copy, Clone)]
pub struct WrBackpressure(bool);

impl WrBackpressure {
    #[inline]
    
    pub fn enabled(&self) -> bool { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
pub struct Error<E> {
    err: E,
}

impl<E> Error<E> {
    pub fn new(err: E) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn get_ref(&self) -> &E { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn into(self) -> E { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
pub struct ProtocolError {
    err: error::ProtocolError,
}

impl ProtocolError {
    pub fn new(err: error::ProtocolError) -> Self { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn get_ref(&self) -> &error::ProtocolError { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn into(self) -> error::ProtocolError { panic!("STUB: not implemented") }
}

#[derive(Debug)]

pub struct PeerGone(pub(crate) Option<io::Error>);

impl PeerGone {
    #[inline]
    
    pub fn err(&self) -> Option<&io::Error> { panic!("STUB: not implemented") }

    #[inline]
    
    pub fn into(self) -> Option<io::Error> { panic!("STUB: not implemented") }
}

#[derive(Debug)]
pub struct DefaultControlService<S, E, R, Err = ()>(PhantomData<(S, E, R, Err)>);

impl<S, E, R, Err> Default for DefaultControlService<S, E, R, Err> {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl<S, E, R, Err> ServiceFactory<Control<E>, S> for DefaultControlService<S, E, R, Err> {
    type Response = Option<R>;
    type Error = E;
    type InitError = Err;
    type Service = DefaultControlService<S, E, R, ()>;

    async fn create(&self, _: S) -> Result<Self::Service, Self::InitError> { panic!("STUB: not implemented") }
}

impl<S, E, R> Service<Control<E>> for DefaultControlService<S, E, R, ()> {
    type Response = Option<R>;
    type Error = E;

    async fn call(
        &self,
        _: Control<E>,
        _: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        
        assert!(format!("{:?}", WrBackpressure(false)).contains("WrBackpressure"));
        assert!(format!("{:?}", Error { err: () }).contains("Error"));
        assert!(format!("{:?}", PeerGone(None)).contains("PeerGone"));
    }
}
