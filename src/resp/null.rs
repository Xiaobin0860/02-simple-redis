use crate::{RespDecode, RespEncode, RespError, RespResult};

use super::NULL;

// null: "_\r\n"
pub(crate) const PREFIX: u8 = b'_';
const BYTE_SIZE: usize = 3;

#[derive(Debug, Clone, PartialEq)]
pub struct RespNull;

impl RespEncode for RespNull {
    fn encode(&self) -> Vec<u8> {
        NULL.to_vec()
    }

    fn byte_size(&self) -> usize {
        BYTE_SIZE
    }
}

impl RespDecode for RespNull {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        if buf.len() < BYTE_SIZE {
            return Err(RespError::NotComplete);
        }
        if buf.starts_with(NULL) {
            Ok(RespNull)
        } else {
            Err(RespError::InvalidFrame(format!(
                "Invalid null frame: {:?}",
                &buf[0..BYTE_SIZE]
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RespFrame;

    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(RespNull.encode(), NULL);
        let frame: RespFrame = RespNull.into();
        assert_eq!(frame.encode(), NULL);
    }

    #[test]
    fn test_byte_size() {
        assert_eq!(RespNull.byte_size(), 3);
    }

    #[test]
    fn test_decode() {
        assert!(RespNull::decode(NULL).is_ok());
        assert!(RespNull::decode(b"_\r\n").is_ok());
        assert!(RespNull::decode(b"_\r\nextra").is_ok());
        assert_eq!(RespNull::decode(b"_"), Err(RespError::NotComplete));
        assert!(RespNull::decode(b"_\r\r").is_err());
        assert_eq!(RespFrame::decode(NULL), Ok(RespNull.into()));
    }
}
