use super::{
    array, bool, bulk_error, bulk_string, double, integer, map, null, set, simple_error,
    simple_string,
};
use crate::{
    RespArray, RespBulkError, RespBulkString, RespDecode, RespMap, RespNull, RespResult, RespSet,
    SimpleError, SimpleString,
};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(RespEncode)]
#[derive(Debug, PartialEq)]
pub enum RespFrame {
    Array(RespArray),
    Bool(bool),
    BulkError(RespBulkError),
    BulkString(RespBulkString),
    Double(f64),
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
        match *prefix {
            array::PREFIX => RespArray::decode(buf).map(RespFrame::Array),
            bool::PREFIX => bool::decode(buf).map(RespFrame::Bool),
            bulk_error::PREFIX => RespBulkError::decode(buf).map(RespFrame::BulkError),
            bulk_string::PREFIX => RespBulkString::decode(buf).map(RespFrame::BulkString),
            double::PREFIX => f64::decode(buf).map(RespFrame::Double),
            integer::PREFIX => i64::decode(buf).map(RespFrame::Integer),
            map::PREFIX => RespMap::decode(buf).map(RespFrame::Map),
            null::PREFIX => RespNull::decode(buf).map(RespFrame::Null),
            set::PREFIX => RespSet::decode(buf).map(RespFrame::Set),
            simple_error::PREFIX => SimpleError::decode(buf).map(RespFrame::SimpleError),
            simple_string::PREFIX => SimpleString::decode(buf).map(RespFrame::SimpleString),
            _ => Err(crate::RespError::InvalidFrameType(format!(
                "Invalid frame type: {prefix}",
            ))),
        }
    }
}
