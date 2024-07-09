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
    println!("{:#?}", args);
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
    println!("{:?}", extract_opts);
    // for arg in args.files.iter() {
    //     let extractor = extractor::create_extractor(arg).unwrap();
    //     let target = arg.to_path_buf();
    // }
    // Ok(())
    let file = args.files.first().unwrap();
    println!("{:?}", file);
    match extractor::create_extractor(file) {
        Ok(extractor) => extractor.perform(file.to_path_buf(), &extract_opts),
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
            cli::CliArgs::parse_from(&["unicom", "-f", "zip", "-r", "-c", "8", "file1", "file2"]);
        assert_eq!(
            opts.files,
            vec![PathBuf::from("file1"), PathBuf::from("file2")]
        );
        assert_eq!(opts.count, 8);
    }
}
