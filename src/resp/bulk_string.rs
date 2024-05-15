use crate::{RespDecode, RespEncode, RespResult};

// bulk string: "$<length>\r\n<data>\r\n"
pub(crate) const PREFIX: u8 = b'$';

#[derive(Debug, PartialEq)]
pub struct RespBulkString {}

impl RespEncode for RespBulkString {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn byte_size(&self) -> usize {
        todo!()
    }
}

impl RespDecode for RespBulkString {
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
