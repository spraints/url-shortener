use std::path::PathBuf;

use clap::Parser;
use tracing::Level;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, short)]
    pub config: PathBuf,

    #[arg(long, short, default_value = "127.0.0.1:3000")]
    pub addr: String,

    #[arg(long, default_value = "info")]
    pub log_level: Level,
}
