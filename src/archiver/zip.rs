use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use zip::result::ZipResult;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::archiver::{ArchiveOpts, Archiver};
use crate::cli::Format;
// use crate::format::Format;

pub(super) struct ZipArchiver {}

impl Archiver for ZipArchiver {
    fn perform(&self, inout: &ArchiveOpts) -> Result<(), Box<dyn Error>> {
        let files = inout.targets();
        perform_archive(files, inout.recursive)?;
        Ok(())
    }

    fn format(&self) -> Format {
        Format::Zip
    }
}

fn perform_archive(files: Vec<PathBuf>, recursive: bool) -> ZipResult<()> {
    let mut zw = ZipWriter::new(File::create("test.zip")?);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for file in files {
        if file.is_dir() && recursive {
            let _ = dir_archive(&mut zw, file.to_path_buf(), options);
        } else {
            let _ = file_archive(&mut zw, file.to_path_buf(), options);
        }
    }

    zw.flush()?;
    zw.finish()?;

    Ok(())
}

fn file_archive<W: Write + Seek>(
    zw: &mut ZipWriter<W>,
    mut target: PathBuf,
    options: SimpleFileOptions,
) -> Result<(), Box<dyn Error>> {
    let mut bytes: Vec<u8> = vec![];
    let mut r = File::open(&target)?;
    let _ = r.read_to_end(&mut bytes)?;

    let _ = zw.start_file_from_path(&mut target, options);
    let _ = zw.write(&bytes);

    Ok(())
}

fn dir_archive<W: Write + Seek>(
    zw: &mut ZipWriter<W>,
    target: PathBuf,
    options: SimpleFileOptions,
) -> Result<(), Box<dyn Error>> {
    for entry in target.read_dir()? {
        println!("{:?}", entry);
        match entry {
            Ok(directory) => {
                let files = directory.path();
                if files.is_dir() {
                    let _ = dir_archive(zw, files, options)?;
                } else {
                    let _ = file_archive(zw, files, options);
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    }
    Ok(())
}
