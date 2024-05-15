use crate::{RespDecode, RespEncode, RespResult};

// map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
pub(crate) const PREFIX: u8 = b'%';

#[derive(Debug, PartialEq)]
pub struct RespMap {}

impl RespEncode for RespMap {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn byte_size(&self) -> usize {
        todo!()
    }
}

impl RespDecode for RespMap {
    fn decode(_buf: &[u8]) -> RespResult<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_encode() {}

    #[test]
    fn test_byte_size() {}

    #[test]
    fn test_decode() {}
}
