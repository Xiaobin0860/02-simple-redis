use enum_dispatch::enum_dispatch;

use crate::{RespBulkError, RespBulkString, RespMap, RespSimpleError};

#[enum_dispatch(RespEncode)]
#[derive(Debug)]
pub enum RespFrame {
    BulkError(RespBulkError),
    SimpleError(RespSimpleError),
    BulkString(RespBulkString),
    Map(RespMap),
}
