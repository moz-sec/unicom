use clap::Parser;

mod cli;

fn main() {
    let args = cli::Args::parse();

    println!("{:?}", args);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_run() {
        let opts =
            cli::Args::parse_from(&["unicom", "-f", "zip", "-r", "-c", "8", "file1", "file2"]);
        assert_eq!(
            opts.files,
            vec![PathBuf::from("file1"), PathBuf::from("file2")]
        );
        assert_eq!(opts.count, 8);
    }
}
