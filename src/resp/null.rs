use crate::{RespDecode, RespEncode, RespResult};

// null: "_\r\n"
#[allow(dead_code)]
const PREFIX: u8 = b'_';

#[derive(Debug, PartialEq)]
pub struct RespNull;

impl RespEncode for RespNull {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn byte_size(&self) -> usize {
        todo!()
    }
}

impl RespDecode for RespNull {
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