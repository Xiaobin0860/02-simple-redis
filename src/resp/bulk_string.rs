use super::CRLF;
use crate::{resp::read_len, RespDecode, RespEncode, RespError, RespResult};

// bulk string: "$<length>\r\n<data>\r\n"
pub(crate) const PREFIX: u8 = b'$';

#[derive(Debug, PartialEq, Hash)]
pub struct BulkString(pub(crate) Vec<u8>);
impl BulkString {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        BulkString(data.into())
    }
}

impl RespEncode for BulkString {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.byte_size());
        buf.push(PREFIX);
        buf.extend_from_slice(self.0.len().to_string().as_bytes());
        buf.extend_from_slice(CRLF);
        buf.extend_from_slice(self.as_ref());
        buf.extend_from_slice(CRLF);
        buf
    }

    fn byte_size(&self) -> usize {
        self.0.len() + self.0.len().to_string().len() + 5
    }
}

impl RespDecode for BulkString {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        let (len, offset) = read_len(PREFIX, buf)?;
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

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
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
    }
}
