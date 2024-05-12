use crate::{RespDecode, RespEncode, RespError, RespResult};

// boolean: "#<t|f>\r\n"
#[allow(dead_code)]
const PREFIX: u8 = b'#';
const BYTE_SIZE: usize = 4;
const TRUE: &[u8] = b"#t\r\n";
const FALSE: &[u8] = b"#f\r\n";

impl RespEncode for bool {
    fn encode(&self) -> Vec<u8> {
        if *self {
            TRUE.to_vec()
        } else {
            FALSE.to_vec()
        }
    }

    fn byte_size(&self) -> usize {
        BYTE_SIZE
    }
}

impl RespDecode for bool {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        if buf.len() < BYTE_SIZE {
            return Err(RespError::NotComplete);
        }

        if buf.starts_with(TRUE) {
            Ok(true)
        } else if buf.starts_with(FALSE) {
            Ok(false)
        } else {
            Err(RespError::InvalidFrame(format!(
                "Invalid boolean frame: {:?}",
                buf
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(true.encode(), TRUE);
        assert_eq!(false.encode(), FALSE);
    }

    #[test]
    fn test_byte_size() {
        assert_eq!(true.byte_size(), 4);
        assert_eq!(false.byte_size(), 4);
    }

    #[test]
    fn test_decode() {
        assert!(bool::decode(TRUE).unwrap());
        assert!(!bool::decode(FALSE).unwrap());
    }
}
