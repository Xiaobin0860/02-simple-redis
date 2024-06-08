mod error;
mod hmap;
mod map;

use crate::{Backend, RespArray, RespFrame, SimpleString};
use enum_dispatch::enum_dispatch;
use error::CommandError;
use std::sync::OnceLock;

pub fn resp_ok() -> &'static RespFrame {
    static RESP_OK: OnceLock<RespFrame> = OnceLock::new();
    RESP_OK.get_or_init(|| SimpleString::new("OK").into())
}

#[enum_dispatch]
pub trait CommandExecutor {
    fn execute(&self, backend: &Backend) -> RespFrame;
}

#[enum_dispatch(CommandExecutor)]
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Get(Get),
    Set(Set),
    HGet(HGet),
    HSet(HSet),
    HGetAll(HGetAll),

    // unrecognized command
    Unrecognized(Unrecognized),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Get {
    pub key: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Set {
    pub key: String,
    pub value: RespFrame,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HGet {
    pub key: String,
    pub field: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HSet {
    pub key: String,
    pub field: String,
    pub value: RespFrame,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HGetAll {
    key: String,
    sort: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unrecognized;

impl TryFrom<RespFrame> for Command {
    type Error = CommandError;

    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        match value {
            RespFrame::Array(array) => array.try_into(),
            _ => Err(CommandError::InvalidCommand(
                "Command must be an array".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for Command {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match value.first() {
            Some(RespFrame::BulkString(ref cmd)) => match cmd.as_ref() {
                b"GET" => Ok(Get::try_from(value)?.into()),
                b"SET" => Ok(Set::try_from(value)?.into()),
                b"HGET" => Ok(HGet::try_from(value)?.into()),
                b"HSET" => Ok(HSet::try_from(value)?.into()),
                _ => Ok(Unrecognized.into()),
            },
            _ => Err(CommandError::InvalidCommand(
                "Command must have a BulkString as the first argument".to_string(),
            )),
        }
    }
}

impl CommandExecutor for Unrecognized {
    fn execute(&self, _backend: &Backend) -> RespFrame {
        resp_ok().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Backend, RespArray, RespDecode, RespNull};
    use anyhow::Result;

    #[test]
    fn test_command() -> Result<()> {
        let buf = b"*2\r\n$3\r\nGET\r\n$5\r\nnnnnn\r\n";
        let frame: RespFrame = RespArray::decode(buf)?.into();
        let cmd: Command = frame.try_into()?;
        assert_eq!(
            cmd,
            Command::Get(Get {
                key: "nnnnn".to_string()
            })
        );
        let backend = Backend::new();
        let ret = cmd.execute(&backend);
        assert_eq!(ret, RespFrame::Null(RespNull));

        Ok(())
    }
}
