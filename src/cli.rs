use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, ValueEnum, Clone)]
pub enum Browser {
    Chrome,
    Firefox,
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long)]
    pub browser: Browser,

    #[arg(long, default_value = "9515")]
    pub port: String,

    #[arg()]
    pub file_path: PathBuf,
}
