use crate::archiver::zip::ZipArchiver;
use crate::cli::{CliArgs, Format};
use std::error::Error;
use std::path::PathBuf;

mod zip;

pub trait Archiver {
    fn perform(&self, inout: &ArchiveOpts) -> Result<(), Box<dyn std::error::Error>>;
    fn format(&self) -> Format;
}

pub fn create_archiver(opts: &ArchiveOpts) -> Result<Box<dyn Archiver>, Box<dyn Error>> {
    match opts.format {
        Format::Zip => Ok(Box::new(ZipArchiver {})),
        _ => Err("Unsupported format".into()),
    }
}

pub struct ArchiveOpts {
    pub format: Format,
    pub files: Vec<PathBuf>,
    pub recursive: bool,
}

impl ArchiveOpts {
    pub fn new(args: &CliArgs) -> Self {
        let files = args.files.clone();
        ArchiveOpts {
            format: args.format.clone(),
            files: files,
            recursive: args.recursive,
        }
    }

    pub fn targets(&self) -> Vec<PathBuf> {
        self.files.clone()
    }
}
