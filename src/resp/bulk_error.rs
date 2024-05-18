use super::{read_len, CRLF};
use crate::{RespDecode, RespEncode, RespError, RespResult};

// bulk error: "!<length>\r\n<error>\r\n"
pub(crate) const PREFIX: u8 = b'!';

#[derive(Debug, PartialEq)]
pub struct BulkError(Vec<u8>);

impl BulkError {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        BulkError(data.into())
    }
}

impl RespEncode for BulkError {
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

impl RespDecode for BulkError {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        let (len, offset) = read_len(PREFIX, buf)?;
        let total_len = len as usize + offset + 2;
        if buf.len() < total_len {
            return Err(RespError::NotComplete);
        }
        if buf[total_len - 2] != CRLF[0] || buf[total_len - 1] != CRLF[1] {
            return Err(RespError::InvalidFrame(format!(
                "Invalid bulk error tail: {:?}",
                &buf[total_len - 2..total_len]
            )));
        }
        Ok(BulkError(buf[offset..total_len - 2].to_vec()))
    }
}

impl AsRef<[u8]> for BulkError {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;

    #[test]
    fn test_encode() {
        let frame: RespFrame = BulkError::new(b"bulk error").into();
        assert_eq!(frame.encode(), b"!10\r\nbulk error\r\n");
    }

    #[test]
    fn test_byte_size() {
        let frame: RespFrame = BulkError::new(b"bulk error").into();
        assert_eq!(frame.byte_size(), 17);
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            RespFrame::decode(b"!10\r\nbulk error\r\n"),
            Ok(BulkError::new(b"bulk error").into()),
        );
        assert_eq!(
            RespFrame::decode(b"!10\r\nbulk error"),
            Err(RespError::NotComplete),
        );
        assert_eq!(
            RespFrame::decode(b"!10\r\nbulk error\r\nextra"),
            Ok(BulkError::new(b"bulk error").into()),
        );
        assert_eq!(
            RespFrame::decode(b"!10\r\nbulk error\n\nextra"),
            Err(RespError::InvalidFrame(
                "Invalid bulk error tail: [10, 10]".to_string()
            )),
        );
    }
}
