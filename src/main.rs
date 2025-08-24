use std::fs;

use clap::Parser;
use config::Config;

mod cli;
mod config;

fn main() {
    let cli::Cli { config } = cli::Cli::parse();
    let f = fs::File::open(&config).unwrap();
    let config: Config = serde_yaml::from_reader(&f).unwrap();
    println!("Hello, world! {config:?}");
}
