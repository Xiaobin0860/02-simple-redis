use std::{ops::Deref, sync::Arc};

use dashmap::DashMap;

use crate::RespFrame;

type Map = DashMap<String, RespFrame>;
type HMap = DashMap<String, Map>;

#[derive(Default)]
pub struct BackendState {
    pub(crate) map: Map,
    pub(crate) hmap: HMap,
}

#[derive(Clone)]
pub struct Backend(Arc<BackendState>);

impl Backend {
    pub fn new() -> Self {
        let s = BackendState::default();
        Backend(Arc::new(s))
    }

    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: RespFrame) {
        self.map.insert(key, value);
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<RespFrame> {
        self.hmap
            .get(key)
            .and_then(|map| map.get(field).map(|v| v.value().clone()))
    }

    pub fn hset(&self, key: String, field: String, value: RespFrame) {
        let map = self.hmap.entry(key).or_default();
        map.insert(field, value);
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Backend {
    type Target = BackendState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
