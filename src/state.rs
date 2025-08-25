use std::collections::HashMap;

use tokio::sync::RwLock;
use tracing::debug;

use crate::config::{Config, RedirectRule};

pub struct AppState {
    config: RwLock<HashMap<String, RedirectRule>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(&self, shortened: &str) -> Option<RedirectRule> {
        let res = self.config.read().await.get(shortened).cloned();
        debug!("{shortened:?} => {res:?}");
        res
    }

    pub async fn set_config(&self, config: Config) {
        let mut res = HashMap::new();
        for rule in config.urls {
            let k = rule.short.clone();
            res.insert(k, rule);
        }
        let mut p = self.config.write().await;
        *p = res;
    }
}
