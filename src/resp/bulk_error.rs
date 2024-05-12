use crate::{RespDecode, RespEncode, RespResult};

// bulk error: "!<length>\r\n<error>\r\n"
#[allow(dead_code)]
const PREFIX: u8 = b'!';

#[derive(Debug, PartialEq)]
pub struct RespBulkError(Vec<u8>);

impl RespEncode for RespBulkError {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn byte_size(&self) -> usize {
        todo!()
    }
}

impl RespDecode for RespBulkError {
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
