use std::ops::Deref;

use crate::{RespDecode, RespEncode, RespFrame, RespResult};

use super::{read_len, CRLF};

// array: "*<number-of-elements>\r\n<element-1>...<element-n>"
//        "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
// null array: "*-1\r\n"
pub(crate) const PREFIX: u8 = b'*';
pub(crate) const NULL: &[u8] = b"*-1\r\n";

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RespArray(Vec<RespFrame>);

impl RespEncode for RespArray {
    fn encode(&self) -> Vec<u8> {
        if self.is_empty() {
            return NULL.to_vec();
        }
        let mut buf = Vec::with_capacity(self.byte_size());
        buf.push(PREFIX);
        buf.extend_from_slice(self.len().to_string().as_bytes());
        buf.extend_from_slice(CRLF);
        for frame in self.iter() {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }

    fn byte_size(&self) -> usize {
        if self.is_empty() {
            return NULL.len();
        }
        let ct = self.len();
        let mut size = 1 + ct.to_string().len() + 2;
        for frame in self.iter() {
            size += frame.byte_size();
        }
        size
    }
}

impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(s.into())
    }
}

impl RespDecode for RespArray {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        let (count, mut offset) = read_len(PREFIX, buf)?;
        if count == -1 {
            return Ok(RespArray::default());
        }
        let mut frames = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let frame = RespFrame::decode(&buf[offset..])?;
            offset += frame.byte_size();
            frames.push(frame);
        }
        Ok(RespArray(frames))
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BulkString, RespError};
    use anyhow::Result;

    #[test]
    fn test_array_encode() {
        let buf = b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n";
        let frame: RespFrame = RespArray::new(vec![
            BulkString::new("set".to_string()).into(),
            BulkString::new("hello".to_string()).into(),
            BulkString::new("world".to_string()).into(),
        ])
        .into();
        assert_eq!(&frame.encode(), buf);
        assert_eq!(frame.byte_size(), buf.len());

        let frame: RespFrame = RespArray::new(vec![]).into();
        assert_eq!(frame.encode(), NULL);
        let frame: RespFrame = RespArray::default().into();
        assert_eq!(frame.encode(), NULL);
        assert_eq!(NULL.len(), frame.byte_size());
    }

    #[test]
    fn test_array_decode() -> Result<()> {
        let buf = b"*2\r\n$3\r\nset\r\n$5\r\nhello\r\n";
        let frame = RespArray::decode(buf)?;
        assert_eq!(frame, RespArray::new([b"set".into(), b"hello".into()]));
        assert_eq!(frame.byte_size(), buf.len());

        let ret = RespArray::decode(b"*2\r\n$3\r\nset\r\n");
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        let frame = RespFrame::decode(NULL)?;
        assert_eq!(frame, RespArray::default().into());

        Ok(())
    }
}
