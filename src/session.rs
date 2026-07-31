use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

pub struct Session<T, St>(Rc<SessionInner<T, St>>);

struct SessionInner<T, St> {
    st: St,
    sink: T,
}

impl<T, St> Clone for Session<T, St> {
    #[inline]
    fn clone(&self) -> Self { panic!("STUB: not implemented") }
}

impl<T, St> Session<T, St> {
    pub(crate) fn new(st: St, sink: T) -> Self { panic!("STUB: not implemented") }

    #[inline]
    pub fn sink(&self) -> &T { panic!("STUB: not implemented") }
}

impl<T, St> Deref for Session<T, St> {
    type Target = St;

    #[inline]
    fn deref(&self) -> &St { panic!("STUB: not implemented") }
}

impl<T, St> fmt::Debug for Session<T, St> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session() {
        let s = Session::new(42u32, "sink");
        assert_eq!(s.sink(), &"sink");
        assert_eq!(*s, 42u32);
        assert_eq!(format!("{s:?}"), "Session");
    }
}
