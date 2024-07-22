use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::create_dir_all;
use std::io::Read;
use std::{fs::File, path::PathBuf};

use tar::Archive;

use crate::extractor::{Extractor, ExtractorOpts};

pub(super) struct TarGzExtractor {}

impl Extractor for TarGzExtractor {
    fn perform(&self, archive_file: PathBuf, opts: &ExtractorOpts) -> Result<(), Box<dyn Error>> {
        match open_tar_file(&archive_file, |f| GzDecoder::new(f)) {
            Ok(archive) => extract_tar(archive, archive_file, opts),
            Err(e) => Err(e),
        }
    }
}

fn extract_tar<R: Read>(
    mut archive: tar::Archive<R>,
    original: PathBuf,
    opts: &ExtractorOpts,
) -> Result<(), Box<dyn Error>> {
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let path = entry.header().path().unwrap();
        let p = path.clone().to_path_buf();
        if is_filename_mac_finder_file(p.to_path_buf()) {
            continue;
        }

        let dest = opts.destination(&original).join(path);
        if entry.header().entry_type().is_file() {
            create_dir_all(dest.parent().unwrap()).unwrap();
            entry.unpack(dest).unwrap();
        }
    }
    Ok(())
}

fn is_filename_mac_finder_file(path: PathBuf) -> bool {
    let filename = path.file_name().unwrap().to_str().unwrap();
    filename == ".DS_Store" || filename.starts_with("._")
}

fn open_tar_file<F, R: Read>(file: &PathBuf, opener: F) -> Result<Archive<R>, Box<dyn Error>>
where
    F: FnOnce(File) -> R,
{
    let file = match File::open(file) {
        Ok(f) => f,
        Err(e) => return Err(Box::new(e)),
    };
    let writer = opener(file);
    Ok(Archive::new(writer))
}
