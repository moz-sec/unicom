use clap::{Parser, ValueEnum};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Compress or Decompress file name
    #[arg(value_name = "FILE")]
    file: String,

    /// Multiple File Designation
    #[arg(last=true)]
    multiple_files: Vec<String>,

    /// Whether to decompress the input
    #[arg(short, long)]
    decompress: bool,

    /// Put passwords on compressed files
    #[arg(short, long)]
    encrypt: bool,

    #[arg(short, long)]
    #[clap(value_enum, default_value_t=Format::Zip)]
    format: Format,

    /// Whether to recursively greet
    #[arg(short, long)]
    recursive: bool,

    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Number of compressions to perform
    #[arg(short, long, default_value = "6")]
    count: u32,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Gzip,
    Zip,
}

fn hello() -> String {
    "Hello, world!".to_string()
}

fn main() {
    let _args = Args::parse();

    println!("Hello, world!");
    println!("{}", hello());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
