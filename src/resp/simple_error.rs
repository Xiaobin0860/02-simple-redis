use std::str::from_utf8;

use super::{find_crlf, CRLF};
use crate::{RespDecode, RespEncode, RespResult};

// error: "-Error message\r\n"
pub(crate) const PREFIX: u8 = b'-';

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleError(String);

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl RespEncode for SimpleError {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.byte_size());
        buf.push(PREFIX);
        buf.extend_from_slice(self.0.as_bytes());
        buf.extend_from_slice(CRLF);
        buf
    }

    fn byte_size(&self) -> usize {
        self.0.len() + 3
    }
}

impl RespDecode for SimpleError {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        let end = find_crlf(buf).ok_or(super::RespError::NotComplete)?;
        Ok(Self::new(from_utf8(&buf[1..end])?))
    }
}

#[cfg(test)]
mod tests {
    use crate::{RespError, RespFrame};

    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            SimpleError::new("Error message").encode(),
            b"-Error message\r\n"
        );
        let frame: RespFrame = SimpleError::new("Error message").into();
        assert_eq!(frame.encode(), b"-Error message\r\n");
    }

    #[test]
    fn test_byte_size() {
        assert_eq!(SimpleError::new("Error message").byte_size(), 16);
        let frame: RespFrame = SimpleError::new("Error message").into();
        assert_eq!(frame.byte_size(), 16);
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            RespFrame::decode(b"-Error message\r\n"),
            Ok(SimpleError::new("Error message").into()),
        );
        assert_eq!(
            RespFrame::decode(b"-Error message"),
            Err(RespError::NotComplete),
        );
    }
}
