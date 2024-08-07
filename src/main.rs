use clap::Parser;
use std::error::Error;

mod archiver;
mod cli;
mod extractor;
mod format;

fn main() -> Result<(), Box<dyn Error>> {
    match run(cli::CliArgs::parse()) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn run(args: cli::CliArgs) -> Result<(), Box<dyn Error>> {
    // println!("{:#?}", args);
    if args.decompress {
        perform_extract(args)?;
    } else {
        perform_archive(args)?;
    }

    Ok(())
}

fn perform_archive(opts: cli::CliArgs) -> Result<(), Box<dyn Error>> {
    let archive_opts = archiver::ArchiveOpts::new(&opts);
    match archiver::create_archiver(&archive_opts) {
        Ok(archiver) => archiver.perform(&archive_opts),
        Err(_) => Err("An error occurred during archiving".into()),
    }
}

fn perform_extract(args: cli::CliArgs) -> Result<(), Box<dyn Error>> {
    let extract_opts = extractor::ExtractorOpts::new(&args);
    let archive_file = &args.archive_file;
    match extractor::create_extractor(archive_file) {
        Ok(extractor) => extractor.perform(archive_file.to_path_buf(), &extract_opts),
        Err(_) => Err("An error occurred during extracting".into()),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_run() {
        let opts = cli::CliArgs::parse_from(&["unicom", "-f", "zip", "-r", "file1", "file2"]);
        assert_eq!(opts.archive_file, PathBuf::from("file1"));
        assert_eq!(opts.files, vec![PathBuf::from("file2")]);
        assert_eq!(opts.format, cli::Format::Zip);
    }
}
