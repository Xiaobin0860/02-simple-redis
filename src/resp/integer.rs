use crate::{RespDecode, RespEncode, RespResult};

// integer: ":[<+|->]<value>\r\n"
#[allow(dead_code)]
const PREFIX: u8 = b':';

impl RespEncode for i64 {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }

    fn byte_size(&self) -> usize {
        todo!()
    }
}

impl RespDecode for i64 {
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
