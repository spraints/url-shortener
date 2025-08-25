use std::{fs, sync::Arc};

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use clap::Parser;
use http::StatusCode;
use state::AppState;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

mod cli;
mod config;
mod config_loader;
mod state;

#[tokio::main]
async fn main() {
    let cli::Cli {
        addr,
        config,
        log_level,
    } = cli::Cli::parse();

    tracing_subscriber::fmt().with_max_level(log_level).init();

    let state = Arc::new(AppState::new());

    let w = config_loader::watch(state.clone(), &config).unwrap();

    info!("listening on {addr}");
    let app = Router::new()
        .route("/{shortened}", get(redirect))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
