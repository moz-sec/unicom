use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::PathBuf;

use crate::extractor::{Extractor, ExtractorOpts};
use crate::format::Format;

pub(super) struct ZipExtractor {}

impl Extractor for ZipExtractor {
    fn perform(&self, archive_file: PathBuf, opts: &ExtractorOpts) -> Result<(), Box<dyn Error>> {
        println!("Extracting {:?}", archive_file);
        let zip_file = File::open(&archive_file).unwrap();
        let mut zip = zip::ZipArchive::new(zip_file).unwrap();
        let dest_base = opts.destination(&archive_file);
        println!("Extracting to {:?}", dest_base);
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            if file.is_file() {
                let dest = dest_base.join(PathBuf::from(file.name().to_string()));
                create_dir_all(dest.parent().unwrap()).unwrap();
                let mut out = File::create(dest).unwrap();
                copy(&mut file, &mut out).unwrap();
            }
        }
        Ok(())
    }

    fn format(&self) -> Format {
        Format::Zip
    }
}
