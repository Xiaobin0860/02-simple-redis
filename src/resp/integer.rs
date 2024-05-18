use std::str::from_utf8;

use crate::{RespDecode, RespEncode, RespResult};

use super::{find_crlf, CRLF};

// integer: ":[<+|->]<value>\r\n"
pub(crate) const PREFIX: u8 = b':';

impl RespEncode for i64 {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.byte_size());
        buf.push(PREFIX);
        buf.extend_from_slice(to_string(*self).as_bytes());
        buf.extend_from_slice(CRLF);
        buf
    }

    fn byte_size(&self) -> usize {
        to_string(*self).len() + 3
    }
}

impl RespDecode for i64 {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        let end = find_crlf(buf).ok_or(super::RespError::NotComplete)?;
        let s = from_utf8(&buf[1..end])?;
        Ok(s.parse()?)
    }
}

fn to_string(value: i64) -> String {
    let sign = if value < 0 { "" } else { "+" };
    format!("{sign}{value}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;
    use anyhow::Result;

    #[test]
    fn test_integer_encode() {
        let frame: RespFrame = 123.into();
        assert_eq!(frame.encode(), b":+123\r\n");

        let frame: RespFrame = (-123).into();
        assert_eq!(frame.encode(), b":-123\r\n");
    }

    #[test]
    fn test_integer_decode() -> Result<()> {
        let frame = i64::decode(b":+123\r\n")?;
        assert_eq!(frame, 123);

        let frame = i64::decode(b":-123\r\n")?;
        assert_eq!(frame, -123);

        Ok(())
    }
}
