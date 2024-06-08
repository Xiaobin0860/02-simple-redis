use super::{resp_ok, CommandError};
use crate::{Backend, CommandExecutor, Get, RespArray, RespFrame, RespNull, Set};

impl CommandExecutor for Get {
    fn execute(&self, backend: &Backend) -> RespFrame {
        match backend.get(&self.key) {
            Some(value) => value,
            None => RespNull.into(),
        }
    }
}

impl CommandExecutor for Set {
    fn execute(&self, backend: &Backend) -> RespFrame {
        backend.set(self.key.clone(), self.value.clone());
        resp_ok().clone()
    }
}

impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        if let Some(RespFrame::BulkString(key)) = value.get(1) {
            Ok(Self {
                key: key.to_string(),
            })
        } else {
            Err(CommandError::InvalidCommand(
                "GET command must have a key".to_string(),
            ))
        }
    }
}

impl TryFrom<RespArray> for Set {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        if let Some(RespFrame::BulkString(key)) = value.get(1) {
            if let Some(val) = value.get(2) {
                Ok(Self {
                    key: key.to_string(),
                    value: val.clone(),
                })
            } else {
                Err(CommandError::InvalidCommand(
                    "SET command must have a value".to_string(),
                ))
            }
        } else {
            Err(CommandError::InvalidCommand(
                "SET command must have a key".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Backend;
    use anyhow::Result;

    #[test]
    fn test_set_get_command() -> Result<()> {
        let backend = Backend::new();
        let cmd = Set {
            key: "hello".to_string(),
            value: RespFrame::BulkString(b"world".into()),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, resp_ok().clone());

        let cmd = Get {
            key: "hello".to_string(),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RespFrame::BulkString(b"world".into()));

        Ok(())
    }
}
