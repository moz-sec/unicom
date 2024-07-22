use crate::archiver::tar::TarGzArchiver;
use crate::archiver::zip::ZipArchiver;
use crate::cli::{CliArgs, Format};
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

mod tar;
mod zip;

pub trait Archiver {
    fn perform(&self, inout: &ArchiveOpts) -> Result<(), Box<dyn Error>>;
}

pub fn create_archiver(opts: &ArchiveOpts) -> Result<Box<dyn Archiver>, Box<dyn Error>> {
    match opts.format {
        Format::Zip => Ok(Box::new(ZipArchiver {})),
        Format::TarGz => Ok(Box::new(TarGzArchiver {})),
        _ => Err("Unsupported format".into()),
    }
}

pub struct ArchiveOpts {
    pub format: Format,
    pub dest: PathBuf,
    pub targets: Vec<PathBuf>,
    pub recursive: bool,
}

impl ArchiveOpts {
    pub fn new(args: &CliArgs) -> Self {
        let files = args.files.clone();
        ArchiveOpts {
            format: args.format.clone(),
            dest: args.archive_file.clone(),
            targets: files,
            recursive: args.recursive,
        }
    }

    pub fn targets(&self) -> Vec<PathBuf> {
        self.targets.clone()
    }

    /// Returns the destination file for the archive with opening it and create the parent directories.
    /// If the path for destination is a directory or exists and overwrite is false,
    /// this function returns an error.
    pub fn destination(&self) -> Result<File, Box<dyn Error>> {
        let dest_path = self.dest.as_path();
        if dest_path.is_file() && dest_path.exists() {
            return Err(format!("already exists: {:?}", dest_path).into());
        }
        if let Some(parent) = dest_path.parent() {
            if !parent.exists() {
                if let Err(e) = create_dir_all(parent) {
                    return Err(format!("failed to create parent directories: {:?}", e).into());
                }
            }
        }
        match File::create(self.dest.as_path()) {
            Ok(f) => Ok(f),
            Err(e) => Err(format!("failed to create file: {:?}", e).into()),
        }
    }
}
