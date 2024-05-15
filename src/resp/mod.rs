///
/// - 如何 serialize/deserialize Frame
///     - simple string: "+OK\r\n"
///     - error: "-Error message\r\n"
///     - bulk error: "!<length>\r\n<error>\r\n"
///     - integer: ":[<+|->]<value>\r\n"
///     - bulk string: "$<length>\r\n<data>\r\n"
///     - null bulk string: "$-1\r\n"
///     - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
///         - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
///     - null array: "*-1\r\n"
///     - null: "_\r\n"
///     - boolean: "#<t|f>\r\n"
///     - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
///     - big number: "([+|-]<number>\r\n"
///     - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
///     - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
///     - ...
/// - enum RespFrame {}
/// - trait RespEncode / RespDecode (enum dispatch)
/// - bytes trait
///
mod array;
mod bool;
mod bulk_error;
mod bulk_string;
mod double;
mod frame;
mod integer;
mod map;
mod null;
mod set;
mod simple_error;
mod simple_string;

use enum_dispatch::enum_dispatch;
use thiserror::Error;

pub use array::RespArray;
pub use bulk_error::RespBulkError;
pub use bulk_string::RespBulkString;
pub use frame::RespFrame;
pub use map::RespMap;
pub use null::RespNull;
pub use set::RespSet;
pub use simple_error::SimpleError;
pub use simple_string::SimpleString;

const CRLF: &[u8] = b"\r\n";
const NULL: &[u8] = b"_\r\n";
const TRUE: &[u8] = b"#t\r\n";
const FALSE: &[u8] = b"#f\r\n";

#[derive(Debug, Error, PartialEq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length: {0}")]
    InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,

    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("{0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}

pub type RespResult<T> = Result<T, RespError>;

#[enum_dispatch]
pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
    fn byte_size(&self) -> usize;
}

pub trait RespDecode: Sized {
    fn decode(buf: &[u8]) -> RespResult<Self>;
}

fn find_crlf(buf: &[u8]) -> Option<usize> {
    buf.windows(2).position(|w| w == CRLF)
}
