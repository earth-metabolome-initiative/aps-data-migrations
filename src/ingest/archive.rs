//! Shared archive extraction and in-memory text loading helpers.

use std::{
    collections::HashMap,
    error::Error,
    fmt,
    fs::{self, File},
    io::{BufRead, BufReader, Cursor, Read},
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use tar::Archive;
use zip::ZipArchive;

/// Error returned when archive parsing/extraction fails.
#[derive(Debug)]
pub struct IngestArchiveError {
    message: String,
}

impl IngestArchiveError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for IngestArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for IngestArchiveError {}

impl From<std::io::Error> for IngestArchiveError {
    fn from(value: std::io::Error) -> Self {
        Self::new(format!("Archive I/O error: {value}"))
    }
}

impl From<zip::result::ZipError> for IngestArchiveError {
    fn from(value: zip::result::ZipError) -> Self {
        Self::new(format!("ZIP error: {value}"))
    }
}

/// Reads text files from `tar.gz` into map keyed by filename.
pub fn read_tar_gz_text_files(
    bytes: &[u8],
    filter: &dyn Fn(&str) -> bool,
) -> Result<HashMap<String, Vec<String>>, IngestArchiveError> {
    let decoder = GzDecoder::new(Cursor::new(bytes));
    let mut archive = Archive::new(decoder);
    let mut rows_by_file = HashMap::new();

    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?;
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let name = name.to_string();

        if !filter(&name) {
            continue;
        }

        let mut rows = Vec::new();
        let mut reader = BufReader::new(entry);
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line)? == 0 {
                break;
            }
            rows.push(line);
        }

        rows_by_file.insert(name, rows);
    }

    Ok(rows_by_file)
}

/// Reads text files from ZIP into map keyed by path in archive.
pub fn read_zip_text_files(
    bytes: &[u8],
    filter: &dyn Fn(&str) -> bool,
) -> Result<HashMap<String, Vec<String>>, IngestArchiveError> {
    let mut archive = ZipArchive::new(Cursor::new(bytes))?;
    let mut rows_by_file = HashMap::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.is_dir() {
            continue;
        }

        let name = file.name().to_string();
        if !filter(&name) {
            continue;
        }

        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let rows = content.lines().map(|line| format!("{line}\n")).collect();
        rows_by_file.insert(name, rows);
    }

    Ok(rows_by_file)
}

/// Extracts ZIP archive to directory, returning list of file paths.
pub fn extract_zip_to_dir(
    bytes: &[u8],
    output_dir: &Path,
) -> Result<Vec<PathBuf>, IngestArchiveError> {
    fs::create_dir_all(output_dir)?;
    let mut archive = ZipArchive::new(Cursor::new(bytes))?;
    let mut written = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let Some(enclosed_path) = file.enclosed_name().map(|p| p.to_owned()) else {
            continue;
        };
        let outpath = output_dir.join(enclosed_path);

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
            continue;
        }

        if let Some(parent) = outpath.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut outfile = File::create(&outpath)?;
        std::io::copy(&mut file, &mut outfile)?;
        written.push(outpath);
    }

    Ok(written)
}
