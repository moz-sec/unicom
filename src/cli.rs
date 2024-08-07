use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Compressed filename
    pub archive_file: PathBuf,

    /// Target files name
    #[arg(required = true)]
    pub files: Vec<PathBuf>,

    /// Whether to decompress the input
    #[arg(short, long)]
    pub decompress: bool,

    /// Put passwords on compressed files
    #[arg(short, long)]
    encrypt: bool,

    #[arg(short, long)]
    #[clap(value_enum, default_value_t=Format::Zip)]
    pub format: Format,

    /// Whether to recursively greet
    #[arg(short, long)]
    pub recursive: bool,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,

    /// Number of compressions to perform
    #[arg(short, long, default_value = "6")]
    pub count: u32,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Format {
    Zip,
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    TarZstd,
    SevenZ,
    LHA,
    Rar,
}
