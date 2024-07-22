use flate2::write::GzEncoder;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tar::Builder;

use crate::archiver::{ArchiveOpts, Archiver};

pub(super) struct TarGzArchiver {}

impl Archiver for TarGzArchiver {
    fn perform(&self, inout: &ArchiveOpts) -> Result<(), Box<dyn Error>> {
        write_tar(inout, |file| {
            GzEncoder::new(file, flate2::Compression::default())
        })
    }
}

fn write_tar<F, W: Write>(opts: &ArchiveOpts, f: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(File) -> W,
{
    match opts.destination() {
        Err(e) => Err(e),
        Ok(file) => {
            let enc = f(file);
            write_tar_impl(enc, opts.targets(), opts.recursive)
        }
    }
}

fn write_tar_impl<W: Write>(
    file: W,
    targets: Vec<PathBuf>,
    recursive: bool,
) -> Result<(), Box<dyn Error>> {
    let mut builder = Builder::new(file);
    for target in targets {
        let path = target.as_path();
        if path.is_dir() && recursive {
            dir_archive(&mut builder, path.to_path_buf(), recursive)?
        } else {
            file_archive(&mut builder, path.to_path_buf())?
        }
    }
    if let Err(e) = builder.finish() {
        return Err(format!("failed to finalize archive: {:?}", e).into());
    }
    Ok(())
}

fn dir_archive<W: Write>(
    builder: &mut Builder<W>,
    target: PathBuf,
    recursive: bool,
) -> Result<(), Box<dyn Error>> {
    if let Err(e) = builder.append_dir(&target, &target) {
        return Err(format!("failed to append directory: {:?}", e).into());
    }
    for entry in target.read_dir().unwrap() {
        if let Ok(e) = entry {
            let p = e.path();
            if p.is_dir() && recursive {
                dir_archive(builder, e.path(), recursive)?
            } else if p.is_file() {
                file_archive(builder, e.path())?
            }
        }
    }
    Ok(())
}

fn file_archive<W: Write>(builder: &mut Builder<W>, target: PathBuf) -> Result<(), Box<dyn Error>> {
    if let Err(e) = builder.append_path(target) {
        Err(format!("failed to append file: {:?}", e).into())
    } else {
        Ok(())
    }
}
