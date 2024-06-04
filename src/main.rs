use clap::Parser;
use std::error::Error;

mod archiver;
mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    match run(cli::CliOpts::parse()) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn run(args: cli::CliOpts) -> Result<(), Box<dyn Error>> {
    println!("{:#?}", args);
    perform_archive(args)?;

    Ok(())
}

fn perform_archive(opts: cli::CliOpts) -> Result<(), Box<dyn Error>> {
    let archive_opts = archiver::ArchiveOpts::new(&opts);
    match archiver::create_archiver(&archive_opts) {
        Ok(archiver) => archiver.perform(&archive_opts),
        Err(_) => Err("An error occurred during archiving".into()),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_run() {
        let opts =
            cli::CliOpts::parse_from(&["unicom", "-f", "zip", "-r", "-c", "8", "file1", "file2"]);
        assert_eq!(
            opts.files,
            vec![PathBuf::from("file1"), PathBuf::from("file2")]
        );
        assert_eq!(opts.count, 8);
    }
}
