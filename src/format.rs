use crate::cli::Format;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::Display;

pub fn find_format(file_name: Option<&OsStr>) -> Result<Format, Box<dyn Error>> {
    match file_name {
        Some(file_name) => {
            let name = file_name.to_str().unwrap().to_lowercase();
            if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
                return Ok(Format::TarGz);
            } else if name.ends_with(".tar.bz2") || name.ends_with(".tbz2") {
                return Ok(Format::TarBz2);
            } else if name.ends_with(".tar.xz") || name.ends_with(".txz") {
                return Ok(Format::TarXz);
            } else if name.ends_with(".tar.zst") || name.ends_with(".tzst") {
                return Ok(Format::TarZstd);
            } else if name.ends_with(".7z") {
                return Ok(Format::SevenZ);
            } else if name.ends_with(".tar") {
                return Ok(Format::Tar);
            } else if name.ends_with(".lha") || name.ends_with(".lzh") {
                return Ok(Format::LHA);
            } else if name.ends_with(".rar") {
                return Ok(Format::Rar);
            } else if name.ends_with(".zip")
                || name.ends_with(".jar")
                || name.ends_with(".war")
                || name.ends_with(".ear")
            {
                return Ok(Format::Zip);
            } else {
                return Err("Unsupported format".into());
            }
        }
        None => Err("No arguments given. Use --help for usage.".into()),
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Zip => write!(f, "Zip"),
            Format::Tar => write!(f, "Tar"),
            Format::TarGz => write!(f, "TarGz"),
            Format::TarBz2 => write!(f, "TarBz2"),
            Format::TarXz => write!(f, "TarXz"),
            Format::TarZstd => write!(f, "TarZstd"),
            Format::SevenZ => write!(f, "SevenZ"),
            Format::LHA => write!(f, "LHA"),
            Format::Rar => write!(f, "Rar"),
        }
    }
}
