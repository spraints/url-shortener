use std::{fs, sync::Arc};

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use clap::Parser;
use config::{Config, RedirectRule};
use http::StatusCode;
use tokio::sync::RwLock;

mod cli;
mod config;

#[tokio::main]
async fn main() {
    let cli::Cli { addr, config } = cli::Cli::parse();
    let f = fs::File::open(&config).unwrap();
    let config: Config = serde_yaml::from_reader(&f).unwrap();
    let config = RwLock::new(config);

    let state = Arc::new(AppState { config });

    println!("listening on {addr}");
    let app = Router::new()
        .route("/{shortened}", get(redirect))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct AppState {
    config: RwLock<Config>,
}

impl AppState {
    async fn get(&self, shortened: &str) -> Option<RedirectRule> {
        let cfg = self.config.read().await;
        for r in &cfg.urls {
            if r.short == shortened {
                return Some(r.clone());
            }
        }
        None
    }
}

async fn redirect(Path(shortened): Path<String>, State(config): State<Arc<AppState>>) -> Response {
    match config.get(&shortened).await {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(rule) => {
            let uri: String = rule.long.to_string();
            Redirect::temporary(&uri).into_response()
        }
    }
}
