use std::fs;

use axum::{routing::get, Router};
use clap::Parser;
use config::Config;

mod cli;
mod config;

#[tokio::main]
async fn main() {
    let cli::Cli { addr, config } = cli::Cli::parse();
    let f = fs::File::open(&config).unwrap();
    let config: Config = serde_yaml::from_reader(&f).unwrap();
    println!("Hello, world! {config:?}");

    println!("listening on {addr}");
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
