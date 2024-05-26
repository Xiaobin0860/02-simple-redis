use super::read_len;
use crate::{RespDecode, RespEncode, RespFrame, RespResult};
use std::ops::{Deref, DerefMut};

pub(crate) const PREFIX: u8 = b'~';

// set: "~<number-of-elements>\r\n<element-1>...<element-n>"
// 目前使用Vec实现，不使用HashSet是因为目前RespFrame不能直接实现Eq和Hash
#[derive(Debug, PartialEq)]
pub struct RespSet(Vec<RespFrame>);

impl RespSet {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespSet(s.into())
    }
}

impl RespEncode for RespSet {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.byte_size());
        buf.push(PREFIX);
        buf.extend_from_slice(self.len().to_string().as_bytes());
        buf.extend_from_slice(b"\r\n");
        for frame in self.iter() {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }

    fn byte_size(&self) -> usize {
        //%<number-of-entries>\r\n
        let mut size = self.len().to_string().len() + 3;
        for v in self.iter() {
            size += v.byte_size();
        }
        size
    }
}

impl RespDecode for RespSet {
    fn decode(_buf: &[u8]) -> RespResult<Self> {
        let (num, mut offset) = read_len(PREFIX, _buf)?;
        let mut frames = Vec::with_capacity(num as usize);
        for _ in 0..num {
            let frame = RespFrame::decode(&_buf[offset..])?;
            offset += frame.byte_size();
            frames.push(frame);
        }
        Ok(RespSet(frames))
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BulkString, RespArray};
    use anyhow::Result;

    #[test]
    fn test_set_encode() {
        let frame: RespFrame = RespSet::new([
            RespArray::new([1234.into(), true.into()]).into(),
            BulkString::new("world".to_string()).into(),
        ])
        .into();
        assert_eq!(
            frame.encode(),
            b"~2\r\n*2\r\n:+1234\r\n#t\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_set_decode() -> Result<()> {
        let buf = b"~2\r\n$3\r\nset\r\n$5\r\nhello\r\n";

        let frame = RespSet::decode(buf)?;
        assert_eq!(
            frame,
            RespSet::new(vec![
                BulkString::new(b"set".to_vec()).into(),
                BulkString::new(b"hello".to_vec()).into()
            ])
        );

        Ok(())
    }
}
