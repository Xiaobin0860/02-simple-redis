use super::{find_crlf, CRLF};
use crate::{RespDecode, RespEncode, RespResult};

// double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
pub(crate) const PREFIX: u8 = b',';

impl RespEncode for f64 {
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

impl RespDecode for f64 {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        let end = find_crlf(buf).ok_or(super::RespError::NotComplete)?;
        let s = std::str::from_utf8(&buf[1..end])?;
        Ok(s.parse()?)
    }
}

fn to_string(f: f64) -> String {
    if f.abs() > 1e+8 || f.abs() < 1e-8 {
        format!("{:+e}", f)
    } else {
        let sign = if f < 0.0 { "" } else { "+" };
        format!("{sign}{f}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;
    use anyhow::Result;

    #[test]
    fn test_double_encode() {
        let frame: RespFrame = 123.456.into();
        assert_eq!(frame.encode(), b",+123.456\r\n");

        let frame: RespFrame = (-123.456).into();
        assert_eq!(frame.encode(), b",-123.456\r\n");

        let frame: RespFrame = 1.23456e+8.into();
        assert_eq!(frame.encode(), b",+1.23456e8\r\n");

        let frame: RespFrame = (-1.23456e-9).into();
        assert_eq!(&frame.encode(), b",-1.23456e-9\r\n");
    }

    #[test]
    fn test_double_decode() -> Result<()> {
        let f = f64::decode(b",123.45\r\n")?;
        assert_eq!(f, 123.45);

        let f = f64::decode(b",+1.23456e-9\r\n")?;
        assert_eq!(f, 1.23456e-9);

        Ok(())
    }
}
