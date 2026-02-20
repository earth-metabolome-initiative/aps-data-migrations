//! Catalogue of Life (Base release) download and CSV export helpers.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use indicatif::ProgressBar;

use crate::ingest::{
    archive::{IngestArchiveError, extract_zip_to_dir, read_zip_text_files},
    csv::{
        IngestCsvError, IngestParseError, deserialize_delimited_line, deserialize_delimited_lines,
        write_lines_to_csv,
    },
    download::{IngestDownloadError, download_bytes},
    progress::progress_style,
};

use super::{
    CATALOGUE_OF_LIFE_API_BASE, CATALOGUE_OF_LIFE_BASE_DATASET_ID, COL_BASE_ARCHIVE_NAME,
    COL_BASE_EXPORT_FORMAT, COL_BASE_FILE_SPECS,
};

pub type ColDownloadError = IngestDownloadError;
pub type ColCsvError = IngestCsvError;
pub type ColArchiveError = IngestArchiveError;
pub type ColParseError = IngestParseError;

#[derive(Debug, Clone)]
pub struct ColExportSummary {
    pub archive_path: PathBuf,
    pub extracted_dir: PathBuf,
    pub normalized_dir: PathBuf,
    pub written_csvs: Vec<PathBuf>,
}

/// Build the CoL Base release export URL for a dataset id.
pub fn col_base_export_url(dataset_id: u64, extended: bool) -> String {
    let extended_query = if extended { "&extended=true" } else { "" };
    format!(
        "{CATALOGUE_OF_LIFE_API_BASE}/dataset/{dataset_id}/export.zip?format={COL_BASE_EXPORT_FORMAT}{extended_query}"
    )
}

/// Download CoL Base release archive bytes.
pub fn download_col_base_archive(dataset_id: u64) -> Result<Vec<u8>, ColDownloadError> {
    let primary_url = col_base_export_url(dataset_id, true);
    match download_bytes(&primary_url, "CoL Base release (extended)") {
        Ok(bytes) => Ok(bytes),
        Err(_) => {
            let fallback_url = col_base_export_url(dataset_id, false);
            download_bytes(&fallback_url, "CoL Base release")
        }
    }
}

/// Downloads Base release and extracts all tabular lines from ZIP.
pub fn download_col_base_tabular_lines(
    dataset_id: u64,
) -> Result<HashMap<String, Vec<String>>, ColDownloadError> {
    let archive = download_col_base_archive(dataset_id)?;
    read_zip_text_files(&archive, &|name| {
        name.ends_with(".tsv") || name.ends_with(".csv") || name.ends_with(".txt")
    })
    .map_err(|err| ColDownloadError::from(std::io::Error::other(err.to_string())))
}

/// Deserializes one CoL TSV row using the given header names.
pub fn deserialize_col_tsv_line<T>(line: &str, headers: &[&str]) -> Result<T, ColParseError>
where
    T: serde::de::DeserializeOwned,
{
    deserialize_delimited_line(line, headers, b'\t')
}

/// Deserializes many CoL TSV rows using the given header names.
pub fn deserialize_col_tsv_lines<T>(
    lines: &[String],
    headers: &[&str],
) -> Result<Vec<T>, ColParseError>
where
    T: serde::de::DeserializeOwned,
{
    deserialize_delimited_lines(lines, headers, b'\t', "Deserializing CoL rows")
}

fn find_rows_for_spec<'a>(
    rows_by_file: &'a HashMap<String, Vec<String>>,
    file_name: &str,
) -> Option<&'a Vec<String>> {
    rows_by_file.iter().find_map(|(path, rows)| {
        if path.ends_with(file_name) {
            Some(rows)
        } else {
            None
        }
    })
}

/// Downloads CoL Base release and writes raw + normalized outputs.
pub fn download_col_base_and_export_csvs(
    output_root: &Path,
    dataset_id: u64,
) -> Result<ColExportSummary, ColCsvError> {
    let raw_dir = output_root.join("raw");
    let extracted_dir = raw_dir.join("extracted");
    let normalized_dir = output_root.join("normalized");

    fs::create_dir_all(&raw_dir)?;
    fs::create_dir_all(&normalized_dir)?;

    let archive_bytes = download_col_base_archive(dataset_id)
        .map_err(|err| ColCsvError::new(format!("download failed: {err}")))?;

    let archive_path = raw_dir.join(COL_BASE_ARCHIVE_NAME);
    fs::write(&archive_path, &archive_bytes)?;

    extract_zip_to_dir(&archive_bytes, &extracted_dir)
        .map_err(|err| ColCsvError::new(format!("extract failed: {err}")))?;

    let rows_by_file = read_zip_text_files(&archive_bytes, &|name| {
        name.ends_with(".tsv") || name.ends_with(".csv") || name.ends_with(".txt")
    })
    .map_err(|err| ColCsvError::new(format!("read zip failed: {err}")))?;

    let progress = ProgressBar::new(COL_BASE_FILE_SPECS.len() as u64);
    progress.set_style(progress_style(
        "{msg} [{bar:40.magenta/black}] {pos}/{len} ({eta})",
    ));
    progress.set_message("Writing CoL Base CSV artifacts");

    let mut written_csvs = Vec::new();
    for spec in COL_BASE_FILE_SPECS {
        let Some(rows) = find_rows_for_spec(&rows_by_file, spec.file_name) else {
            progress.inc(1);
            continue;
        };

        let stem = Path::new(spec.file_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(spec.file_name)
            .to_ascii_lowercase();
        let output_path = normalized_dir.join(format!("{stem}.csv"));

        let headers = spec.headers;
        write_lines_to_csv(rows, headers, b'\t', &output_path, true)?;
        written_csvs.push(output_path);
        progress.inc(1);
    }

    progress.finish_with_message("CoL CSV export complete");

    Ok(ColExportSummary {
        archive_path,
        extracted_dir,
        normalized_dir,
        written_csvs,
    })
}

/// Convenience wrapper using the default CoL Base dataset id.
pub fn download_default_col_base_and_export_csvs(
    output_root: &Path,
) -> Result<ColExportSummary, ColCsvError> {
    download_col_base_and_export_csvs(output_root, CATALOGUE_OF_LIFE_BASE_DATASET_ID)
}
