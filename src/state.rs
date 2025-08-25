use tokio::sync::RwLock;

use crate::config::{Config, RedirectRule};

pub struct AppState {
    config: RwLock<Config>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let config = RwLock::new(config);
        Self { config }
    }

    pub async fn get(&self, shortened: &str) -> Option<RedirectRule> {
        let cfg = self.config.read().await;
        for r in &cfg.urls {
            if r.short == shortened {
                return Some(r.clone());
            }
        }
        None
    }
}
