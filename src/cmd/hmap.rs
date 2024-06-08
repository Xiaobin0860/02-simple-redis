use super::{resp_ok, CommandError};
use crate::{
    Backend, BulkString, CommandExecutor, HGet, HGetAll, HSet, RespArray, RespFrame, RespNull,
};

impl CommandExecutor for HGet {
    fn execute(&self, backend: &Backend) -> RespFrame {
        match backend.hget(&self.key, &self.field) {
            Some(value) => value,
            None => RespNull.into(),
        }
    }
}

impl CommandExecutor for HSet {
    fn execute(&self, backend: &Backend) -> RespFrame {
        backend.hset(self.key.clone(), self.field.clone(), self.value.clone());
        resp_ok().clone()
    }
}

impl CommandExecutor for HGetAll {
    fn execute(&self, backend: &Backend) -> RespFrame {
        let hmap = backend.hmap.get(&self.key);

        match hmap {
            Some(hmap) => {
                let mut data = Vec::with_capacity(hmap.len());
                for v in hmap.iter() {
                    let key = v.key().to_owned();
                    data.push((key, v.value().clone()));
                }
                if self.sort {
                    data.sort_by(|a, b| a.0.cmp(&b.0));
                }
                let ret = data
                    .into_iter()
                    .flat_map(|(k, v)| vec![BulkString::from(k).into(), v])
                    .collect::<Vec<RespFrame>>();

                RespArray::new(ret).into()
            }
            None => RespArray::new([]).into(),
        }
    }
}

impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        if let Some(RespFrame::BulkString(key)) = value.get(1) {
            if let Some(RespFrame::BulkString(field)) = value.get(2) {
                Ok(Self {
                    key: key.to_string(),
                    field: field.to_string(),
                })
            } else {
                Err(CommandError::InvalidCommand(
                    "HGET command must have a field".to_string(),
                ))
            }
        } else {
            Err(CommandError::InvalidCommand(
                "HGET command must have a key".to_string(),
            ))
        }
    }
}

impl TryFrom<RespArray> for HSet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        if let Some(RespFrame::BulkString(key)) = value.get(1) {
            if let Some(RespFrame::BulkString(field)) = value.get(2) {
                if let Some(val) = value.get(3) {
                    Ok(Self {
                        key: key.to_string(),
                        field: field.to_string(),
                        value: val.clone(),
                    })
                } else {
                    Err(CommandError::InvalidCommand(
                        "HSET command must have a value".to_string(),
                    ))
                }
            } else {
                Err(CommandError::InvalidCommand(
                    "HSET command must have a field".to_string(),
                ))
            }
        } else {
            Err(CommandError::InvalidCommand(
                "HSET command must have a key".to_string(),
            ))
        }
    }
}

impl TryFrom<RespArray> for HGetAll {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        if let Some(RespFrame::BulkString(key)) = value.get(1) {
            Ok(Self {
                key: key.to_string(),
                sort: false,
            })
        } else {
            Err(CommandError::InvalidCommand(
                "HGETALL command must have a key".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BulkString;

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_hset_hget_hgetall_commands() -> Result<()> {
        let backend = crate::Backend::new();
        let cmd = HSet {
            key: "map".to_string(),
            field: "hello".to_string(),
            value: RespFrame::BulkString(b"world".into()),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, resp_ok().clone());

        let cmd = HSet {
            key: "map".to_string(),
            field: "hello1".to_string(),
            value: RespFrame::BulkString(b"world1".into()),
        };
        cmd.execute(&backend);

        let cmd = HGet {
            key: "map".to_string(),
            field: "hello".to_string(),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RespFrame::BulkString(b"world".into()));

        let cmd = HGetAll {
            key: "map".to_string(),
            sort: true,
        };
        let result = cmd.execute(&backend);

        let expected = RespArray::new([
            BulkString::from("hello").into(),
            BulkString::from("world").into(),
            BulkString::from("hello1").into(),
            BulkString::from("world1").into(),
        ]);
        assert_eq!(result, expected.into());
        Ok(())
    }
}
