use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::config::{Config, RedirectRule};

pub struct AppState {
    config: RwLock<HashMap<String, RedirectRule>>,
    hosts: RwLock<HashMap<String, HashMap<String, RedirectRule>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: RwLock::new(HashMap::new()),
            hosts: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(&self, host: &str, shortened: &str) -> Option<RedirectRule> {
        {
            if let Some(rr) = self.config.read().await.get(shortened) {
                return Some(rr.clone());
            }
        }

        {
            if let Some(host) = self.hosts.read().await.get(host) {
                if let Some(rr) = host.get(shortened) {
                    return Some(rr.clone());
                }
            }
        }

        None
    }

    pub async fn set_config(&self, config: Config) {
        let global = index_rules(config.urls);
        let mut p = self.config.write().await;
        *p = global;

        let mut hosts = HashMap::new();
        for host in config.hosts {
            hosts.insert(host.name, index_rules(host.urls));
        }
        let mut p = self.hosts.write().await;
        *p = hosts;
    }
}

fn index_rules(rules: Vec<RedirectRule>) -> HashMap<String, RedirectRule> {
    let mut res = HashMap::new();
    for rule in rules {
        let k = rule.short.clone();
        res.insert(k, rule);
    }
    res
}
