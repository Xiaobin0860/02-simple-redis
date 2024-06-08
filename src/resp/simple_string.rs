use std::{
    ops::Deref,
    str::{from_utf8, FromStr},
};

use crate::{RespDecode, RespEncode, RespError, RespResult};

use super::{find_crlf, CRLF};

// simple string: "+OK\r\n"
pub(crate) const PREFIX: u8 = b'+';

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleString(pub(crate) String);

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl RespEncode for SimpleString {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.byte_size());
        buf.push(PREFIX);
        buf.extend_from_slice(self.0.as_bytes());
        buf.extend_from_slice(CRLF);
        buf
    }

    fn byte_size(&self) -> usize {
        self.len() + 3
    }
}

impl RespDecode for SimpleString {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        // 认为都从 RespFrame::decode 调用，不检查长度和前缀
        let end = find_crlf(buf).ok_or(RespError::NotComplete)?;
        Ok(Self::new(from_utf8(&buf[1..end])?))
    }
}

impl From<&str> for SimpleString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl FromStr for SimpleString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<String> for SimpleString {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl Deref for SimpleString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RespError, RespFrame};

    const SIMPLE_OK: &[u8] = b"+OK\r\n";

    #[test]
    fn test_encode() {
        assert_eq!(SimpleString::new("OK").encode(), SIMPLE_OK);
        // into: enum_dispatch impl From for each variant
        let frame: RespFrame = SimpleString::new("OK").into();
        assert_eq!(frame.encode(), SIMPLE_OK);
    }

    #[test]
    fn test_byte_size() {
        assert_eq!(SimpleString::new("OK").byte_size(), 5);
        let frame = RespFrame::from(SimpleString::new("OK"));
        assert_eq!(frame.byte_size(), 5);
    }

    #[test]
    fn test_decode() {
        assert_eq!(SimpleString::decode(SIMPLE_OK), Ok(SimpleString::new("OK")));
        assert_eq!(
            SimpleString::decode(b"+OK\r\nextra"),
            Ok(SimpleString::new("OK"))
        );
        assert_eq!(SimpleString::decode(b"+OK\r"), Err(RespError::NotComplete));
        assert_eq!(
            RespFrame::decode(SIMPLE_OK),
            Ok(SimpleString::new("OK").into())
        );
    }
}
