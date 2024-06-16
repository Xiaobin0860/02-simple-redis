use super::{
    array, bool, bulk_error, bulk_string, double, integer, map, null, set, simple_error,
    simple_string,
};
use crate::{
    BulkError, BulkString, RespArray, RespDecode, RespError, RespMap, RespNull, RespResult,
    RespSet, SimpleError, SimpleString,
};
use enum_dispatch::enum_dispatch;
use tracing::debug;

#[enum_dispatch(RespEncode)]
#[derive(Debug, Clone, PartialEq)]
pub enum RespFrame {
    Array(RespArray),
    Bool(bool),
    BulkError(BulkError),
    BulkString(BulkString),
    Double(f64), //f64不能直接实现Eq和Hash，将来可以包装成自定义类型
    Integer(i64),
    Map(RespMap),
    Null(RespNull),
    Set(RespSet),
    SimpleError(SimpleError),
    SimpleString(SimpleString),
}

impl RespDecode for RespFrame {
    fn decode(buf: &[u8]) -> RespResult<Self> {
        let prefix = buf.first().ok_or(crate::RespError::NotComplete)?;
        debug!("Decoding frame: {buf:?}");
        match *prefix {
            array::PREFIX => RespArray::decode(buf).map(RespFrame::Array),
            bool::PREFIX => bool::decode(buf).map(RespFrame::Bool),
            bulk_error::PREFIX => BulkError::decode(buf).map(RespFrame::BulkError),
            bulk_string::PREFIX => BulkString::decode(buf).map(RespFrame::BulkString),
            double::PREFIX => f64::decode(buf).map(RespFrame::Double),
            integer::PREFIX => i64::decode(buf).map(RespFrame::Integer),
            map::PREFIX => RespMap::decode(buf).map(RespFrame::Map),
            null::PREFIX => RespNull::decode(buf).map(RespFrame::Null),
            set::PREFIX => RespSet::decode(buf).map(RespFrame::Set),
            simple_error::PREFIX => SimpleError::decode(buf).map(RespFrame::SimpleError),
            simple_string::PREFIX => SimpleString::decode(buf).map(RespFrame::SimpleString),
            _ => Err(RespError::InvalidFrameType(format!(
                "Invalid frame type: {prefix}",
            ))),
        }
    }
}

impl From<&str> for RespFrame {
    fn from(s: &str) -> Self {
        SimpleString(s.to_string()).into()
    }
}

impl From<&[u8]> for RespFrame {
    fn from(s: &[u8]) -> Self {
        BulkString(s.to_vec()).into()
    }
}

impl<const N: usize> From<&[u8; N]> for RespFrame {
    fn from(s: &[u8; N]) -> Self {
        BulkString(s.to_vec()).into()
    }
}
