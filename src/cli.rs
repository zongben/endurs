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

    #[arg()]
    pub file_path: PathBuf,
}
