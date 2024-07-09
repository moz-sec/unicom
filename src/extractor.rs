use crate::cli::CliArgs;
use crate::extractor::zip::ZipExtractor;
use crate::format::{find_format, Format};
use std::error::Error;
use std::path::PathBuf;

mod zip;

pub trait Extractor {
    fn perform(
        &self,
        archive_file: PathBuf,
        opts: &ExtractorOpts,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn format(&self) -> Format;
}

pub fn create_extractor(file: &PathBuf) -> Result<Box<dyn Extractor>, Box<dyn Error>> {
    let format = find_format(file.file_name());
    match format {
        Ok(format) => {
            return match format {
                Format::Zip => Ok(Box::new(ZipExtractor {})),
                Format::Unknown(s) => Err(format!("{}: unsupported format", s).into()),
                _ => todo!(),
            }
        }
        _ => Err("Unsupported format".into()),
    }
}

#[derive(Debug)]
pub struct ExtractorOpts {
    pub dest: PathBuf,
    pub use_archive_name_dir: bool,
    pub verbose: bool,
}

impl ExtractorOpts {
    pub fn new(args: &CliArgs) -> ExtractorOpts {
        let d = args.files.clone().into_iter().next();
        ExtractorOpts {
            dest: d.unwrap_or_else(|| PathBuf::from(".")),
            use_archive_name_dir: false,
            verbose: args.verbose,
        }
    }

    pub fn destination(&self, target: &PathBuf) -> PathBuf {
        if self.use_archive_name_dir {
            let file_name = target.file_name().unwrap().to_str().unwrap();
            let ext = target.extension().unwrap().to_str().unwrap();
            let dir_name = file_name
                .trim_end_matches(ext)
                .trim_end_matches(".")
                .to_string();
            self.dest.join(dir_name)
        } else {
            let dir_name = target.file_stem().unwrap().to_str().unwrap();
            PathBuf::from(dir_name)
        }
    }
}
