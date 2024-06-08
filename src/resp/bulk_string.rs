use std::{
    fmt::{self, Display, Formatter},
    ops::Deref,
};

use super::CRLF;
use crate::{resp::read_len, RespDecode, RespEncode, RespError, RespResult};

pub(crate) const PREFIX: u8 = b'$';
pub(crate) const NULL: &[u8] = b"$-1\r\n";

//bulk string: "$<length>\r\n<data>\r\n"
//null bulk string: "$-1\r\n"
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BulkString(pub(crate) Vec<u8>);
impl BulkString {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        BulkString(data.into())
    }
}

impl RespEncode for BulkString {
    fn encode(&self) -> Vec<u8> {
        if self.is_empty() {
            return NULL.to_vec();
        }
        let mut buf = Vec::with_capacity(self.byte_size());
        buf.push(PREFIX);
        buf.extend_from_slice(self.0.len().to_string().as_bytes());
        buf.extend_from_slice(CRLF);
        buf.extend_from_slice(self.as_ref());
        buf.extend_from_slice(CRLF);
        buf
    }

    fn byte_size(&self) -> usize {
        if self.is_empty() {
            return NULL.len();
        }
        self.len() + self.len().to_string().len() + 5
    }
}

impl RespDecode for BulkString {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        let (len, offset) = read_len(PREFIX, buf)?;
        if len == -1 {
            return Ok(BulkString::default());
        }
        let total_len = len as usize + offset + 2;
        if buf.len() < total_len {
            return Err(RespError::NotComplete);
        }
        if buf[total_len - 2] != CRLF[0] || buf[total_len - 1] != CRLF[1] {
            return Err(RespError::InvalidFrame(format!(
                "Invalid bulk string tail: {:?}",
                &buf[total_len - 2..total_len]
            )));
        }
        Ok(BulkString(buf[offset..total_len - 2].to_vec()))
    }
}

impl From<&[u8]> for BulkString {
    fn from(s: &[u8]) -> Self {
        BulkString::new(s)
    }
}

impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        BulkString::new(s.as_bytes())
    }
}

impl Display for BulkString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl From<String> for BulkString {
    fn from(s: String) -> Self {
        BulkString::new(s)
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(s: &[u8; N]) -> Self {
        BulkString::new(s)
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::RespFrame;

    use super::*;

    #[test]
    fn test_encode() {
        let frame: RespFrame = BulkString::new(b"bulk string").into();
        assert_eq!(frame.encode(), b"$11\r\nbulk string\r\n");

        let frame: RespFrame = BulkString::default().into();
        assert_eq!(frame.encode(), NULL);
        assert_eq!(5, frame.byte_size());
    }

    #[test]
    fn test_byte_size() {
        let frame: RespFrame = BulkString::new(b"bulk string").into();
        assert_eq!(frame.byte_size(), 18);
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            RespFrame::decode(b"$11\r\nbulk string\r\n"),
            Ok(BulkString::new(b"bulk string").into()),
        );
        assert_eq!(
            RespFrame::decode(b"$11\r\nbulk string\r\nabc"),
            Ok(BulkString::new(b"bulk string").into()),
        );
        assert_eq!(
            RespFrame::decode(b"$11\r\nbulk string"),
            Err(RespError::NotComplete),
        );
        assert_eq!(
            RespFrame::decode(b"$11\r\nbulk string\r"),
            Err(RespError::NotComplete),
        );
        assert_eq!(
            RespFrame::decode(b"$11\r\nbulk string\n\nextra"),
            Err(RespError::InvalidFrame(
                "Invalid bulk string tail: [10, 10]".to_string()
            )),
        );

        assert_eq!(RespFrame::decode(NULL), Ok(BulkString::default().into()),);
    }
}
