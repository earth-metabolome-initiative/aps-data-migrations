//! NCBI-specific utilities built on top of shared ingestion helpers.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Serialize, de::DeserializeOwned};

use crate::ingest::{
    archive::read_tar_gz_text_files,
    csv::{
        IngestCsvError, IngestParseError, deserialize_delimited_lines,
        write_records_to_csv as ingest_write_records_to_csv, write_rows_to_csv,
    },
    download::{IngestDownloadError, download_bytes},
};

use super::{NCBI_TAXONOMY_BASE_URL, NCBI_TAXONOMY_NEW_TAXDUMP_PATH, NEW_TAXDUMP_FILE_SPECS};

pub type DmpDeserializeError = IngestParseError;
pub type NcbiTaxonomyDownloadError = IngestDownloadError;
pub type NcbiTaxonomyCsvError = IngestCsvError;

/// Builds an absolute taxonomy URL from a relative path.
pub fn ncbi_taxonomy_url(path: &str) -> String {
    format!("{NCBI_TAXONOMY_BASE_URL}{path}")
}

/// Downloads a taxonomy archive by relative path and returns raw bytes.
pub fn download_ncbi_taxdump_archive(path: &str) -> Result<Vec<u8>, NcbiTaxonomyDownloadError> {
    let url = ncbi_taxonomy_url(path);
    download_bytes(&url, path)
}

/// Downloads the currently selected `new_taxdump` archive.
pub fn download_new_taxdump_archive() -> Result<Vec<u8>, NcbiTaxonomyDownloadError> {
    download_ncbi_taxdump_archive(NCBI_TAXONOMY_NEW_TAXDUMP_PATH)
}

/// Converts an NCBI DMP row from `\t|\t`/`\t|` delimiters to a TSV column vector.
fn normalize_dmp_line(line: &str) -> Result<Vec<String>, DmpDeserializeError> {
    let line = line.trim_end_matches(['\n', '\r']);
    let without_terminal_pipe = line
        .strip_suffix("\t|")
        .or_else(|| line.strip_suffix('|'))
        .ok_or_else(|| DmpDeserializeError::new("DMP line is missing terminal '|'."))?;

    Ok(without_terminal_pipe
        .split("\t|\t")
        .map(|field| field.trim().to_string())
        .collect())
}

/// Deserializes one NCBI DMP row into a target struct using provided header names.
pub fn deserialize_dmp_line<T>(line: &str, headers: &[&str]) -> Result<T, DmpDeserializeError>
where
    T: DeserializeOwned,
{
    let normalized = normalize_dmp_line(line)?;
    let tsv_line = normalized.join("\t");
    crate::ingest::csv::deserialize_delimited_line::<T>(&tsv_line, headers, b'\t')
}

/// Deserializes a collection of raw DMP lines using a provided header specification.
pub fn deserialize_dmp_lines<T>(
    lines: &[String],
    headers: &[&str],
) -> Result<Vec<T>, DmpDeserializeError>
where
    T: DeserializeOwned,
{
    let normalized_lines: Vec<String> = lines
        .iter()
        .map(|line| normalize_dmp_line(line).map(|cols| cols.join("\t")))
        .collect::<Result<Vec<_>, _>>()?;

    deserialize_delimited_lines(&normalized_lines, headers, b'\t', "Deserializing rows")
}

/// Extracts all rows from each `.dmp` file in a `new_taxdump.tar.gz` archive.
pub fn dmp_lines_from_new_taxdump_archive(
    archive_bytes: &[u8],
) -> Result<HashMap<String, Vec<String>>, NcbiTaxonomyDownloadError> {
    read_tar_gz_text_files(archive_bytes, &|name| name.ends_with(".dmp"))
        .map_err(|err| NcbiTaxonomyDownloadError::from(std::io::Error::other(err.to_string())))
}

/// Downloads `new_taxdump` and returns all rows for each `.dmp` file.
pub fn download_new_taxdump_dmp_lines()
-> Result<HashMap<String, Vec<String>>, NcbiTaxonomyDownloadError> {
    let archive = download_new_taxdump_archive()?;
    dmp_lines_from_new_taxdump_archive(&archive)
}

/// Downloads `new_taxdump` and returns first rows for each `.dmp` file.
pub fn download_new_taxdump_dmp_first_lines()
-> Result<HashMap<String, String>, NcbiTaxonomyDownloadError> {
    let rows = download_new_taxdump_dmp_lines()?;
    Ok(rows
        .into_iter()
        .filter_map(|(file_name, file_rows)| file_rows.first().cloned().map(|row| (file_name, row)))
        .collect())
}

/// Writes typed records to a CSV file using serde serialization.
pub fn write_records_to_csv_wrapped<T>(
    records: &[T],
    output_path: &Path,
) -> Result<(), NcbiTaxonomyCsvError>
where
    T: Serialize,
{
    ingest_write_records_to_csv(records, output_path)
}

/// Writes a raw DMP file to a normalized CSV with explicit headers.
pub fn write_dmp_lines_to_csv(
    lines: &[String],
    headers: &[&str],
    output_path: &Path,
) -> Result<(), NcbiTaxonomyCsvError> {
    let rows: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            normalize_dmp_line(line).map_err(|err| {
                NcbiTaxonomyCsvError::new(format!("line normalization failed: {err}"))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    write_rows_to_csv(&rows, Some(headers), output_path, true)
}

/// Downloads `new_taxdump` and writes one normalized CSV file per known `.dmp` artifact.
pub fn download_new_taxdump_and_write_csvs(
    output_dir: &Path,
) -> Result<Vec<PathBuf>, NcbiTaxonomyCsvError> {
    let dmp_rows = download_new_taxdump_dmp_lines()
        .map_err(|err| NcbiTaxonomyCsvError::new(format!("download failed: {err}")))?;
    let mut written_paths = Vec::new();

    let progress = indicatif::ProgressBar::new(NEW_TAXDUMP_FILE_SPECS.len() as u64);
    progress.set_style(crate::ingest::progress::progress_style(
        "{msg} [{bar:40.magenta/black}] {pos}/{len} ({eta})",
    ));
    progress.set_message("Writing CSV artifacts");

    for spec in NEW_TAXDUMP_FILE_SPECS {
        let rows = dmp_rows.get(spec.file_name).ok_or_else(|| {
            NcbiTaxonomyCsvError::new(format!(
                "missing expected file in archive: {}",
                spec.file_name
            ))
        })?;

        let mut output_path = output_dir.join(spec.file_name);
        output_path.set_extension("csv");
        write_dmp_lines_to_csv(rows, spec.headers, &output_path)?;
        written_paths.push(output_path);
        progress.inc(1);
    }

    progress.finish_with_message("CSV export complete");
    Ok(written_paths)
}

/// Backward compatible alias used by integration tests.
pub fn write_records_to_csv<T>(
    records: &[T],
    output_path: &Path,
) -> Result<(), NcbiTaxonomyCsvError>
where
    T: Serialize,
{
    write_records_to_csv_wrapped(records, output_path)
}
