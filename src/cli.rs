use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, short)]
    pub config: PathBuf,
}
