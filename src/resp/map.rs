use crate::{RespDecode, RespEncode, RespFrame, RespResult, SimpleString};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use super::read_len;

pub(crate) const PREFIX: u8 = b'%';

// map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
// only support string keys
#[derive(Debug, PartialEq, Default)]
pub struct RespMap(BTreeMap<String, RespFrame>);

impl RespMap {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RespEncode for RespMap {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.byte_size());
        buf.push(PREFIX);
        buf.extend_from_slice(self.len().to_string().as_bytes());
        buf.extend_from_slice(b"\r\n");
        for (key, value) in self.iter() {
            buf.extend_from_slice(&SimpleString::new(key).encode());
            buf.extend_from_slice(&value.encode());
        }
        buf
    }

    fn byte_size(&self) -> usize {
        //%<number-of-entries>\r\n
        let mut size = self.len().to_string().len() + 3;
        for (key, value) in self.iter() {
            size += key.len() + 3; //+<key>\r\n
            size += value.byte_size();
        }
        size
    }
}

impl RespDecode for RespMap {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        let (num, mut offset) = read_len(PREFIX, buf)?;
        let mut map = BTreeMap::new();
        for _ in 0..num {
            let key = SimpleString::decode(&buf[offset..])?;
            offset += key.byte_size();
            let value = RespFrame::decode(&buf[offset..])?;
            offset += value.byte_size();
            map.insert(key.0, value);
        }
        Ok(Self(map))
    }
}

impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BulkString;
    use anyhow::Result;

    #[test]
    fn test_map_encode() {
        let mut map = RespMap::new();
        map.insert(
            "hello".to_string(),
            BulkString::new("world".to_string()).into(),
        );
        map.insert("foo".to_string(), (-123456.789).into());

        let buf = b"%2\r\n+foo\r\n,-123456.789\r\n+hello\r\n$5\r\nworld\r\n";
        let frame: RespFrame = map.into();
        assert_eq!(&frame.encode(), buf);
        assert_eq!(frame.byte_size(), buf.len());
    }

    #[test]
    fn test_map_decode() -> Result<()> {
        let buf = b"%2\r\n+hello\r\n$5\r\nworld\r\n+foo\r\n$3\r\nbar\r\n";
        let frame = RespMap::decode(buf)?;
        let mut map = RespMap::new();
        map.insert(
            "hello".to_string(),
            BulkString::new(b"world".to_vec()).into(),
        );
        map.insert("foo".to_string(), BulkString::new(b"bar".to_vec()).into());
        assert_eq!(frame, map);
        assert_eq!(frame.byte_size(), buf.len());

        Ok(())
    }
}
