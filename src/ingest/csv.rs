//! Shared CSV normalization, deserialization, and writing helpers.

use std::{
    error::Error,
    fmt,
    fs::{self, File},
    path::Path,
};

use indicatif::ProgressBar;
use serde::{Serialize, de::DeserializeOwned};

use crate::ingest::progress::progress_style;

/// Error returned when line parsing or deserialization fails.
#[derive(Debug)]
pub struct IngestParseError {
    message: String,
}

impl IngestParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for IngestParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for IngestParseError {}

impl From<csv::Error> for IngestParseError {
    fn from(value: csv::Error) -> Self {
        Self::new(format!("CSV parse error: {value}"))
    }
}

/// Error returned when writing CSV outputs fails.
#[derive(Debug)]
pub struct IngestCsvError {
    message: String,
}

impl IngestCsvError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for IngestCsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for IngestCsvError {}

impl From<std::io::Error> for IngestCsvError {
    fn from(value: std::io::Error) -> Self {
        Self::new(format!("CSV file I/O error: {value}"))
    }
}

impl From<csv::Error> for IngestCsvError {
    fn from(value: csv::Error) -> Self {
        Self::new(format!("CSV write error: {value}"))
    }
}

/// Splits one delimited line into fields after trimming line terminators.
pub fn normalize_delimited_line(
    line: &str,
    delimiter: u8,
    trim_fields: bool,
) -> Result<Vec<String>, IngestParseError> {
    let delim = delimiter as char;
    let stripped = line.trim_end_matches(['\n', '\r']);

    let fields = stripped
        .split(delim)
        .map(|field| {
            if trim_fields {
                field.trim().to_string()
            } else {
                field.to_string()
            }
        })
        .collect();

    Ok(fields)
}

/// Deserializes one delimited line into target type using provided headers.
pub fn deserialize_delimited_line<T>(
    line: &str,
    headers: &[&str],
    delimiter: u8,
) -> Result<T, IngestParseError>
where
    T: DeserializeOwned,
{
    let columns = normalize_delimited_line(line, delimiter, true)?;
    let delim = delimiter as char;
    let csv_input = format!(
        "{}\n{}\n",
        headers.join(&delim.to_string()),
        columns.join(&delim.to_string())
    );

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(true)
        .from_reader(csv_input.as_bytes());

    let mut records = reader.deserialize::<T>();
    records
        .next()
        .ok_or_else(|| IngestParseError::new("No record found in line."))?
        .map_err(Into::into)
}

/// Deserializes many delimited lines with progress reporting.
pub fn deserialize_delimited_lines<T>(
    lines: &[String],
    headers: &[&str],
    delimiter: u8,
    progress_label: &str,
) -> Result<Vec<T>, IngestParseError>
where
    T: DeserializeOwned,
{
    let progress = ProgressBar::new(lines.len() as u64);
    progress.set_style(progress_style(
        "{msg} [{bar:40.green/black}] {pos}/{len} ({eta})",
    ));
    progress.set_message(progress_label.to_string());

    let mut records = Vec::with_capacity(lines.len());
    for line in lines {
        records.push(deserialize_delimited_line::<T>(line, headers, delimiter)?);
        progress.inc(1);
    }

    progress.finish_with_message("Deserialization complete");
    Ok(records)
}

/// Writes rows into CSV at output path.
pub fn write_rows_to_csv(
    rows: &[Vec<String>],
    headers: Option<&[&str]>,
    output_path: &Path,
    flexible: bool,
) -> Result<(), IngestCsvError> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(output_path)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .flexible(flexible)
        .from_writer(file);

    if let Some(headers) = headers {
        writer.write_record(headers)?;
    }

    for row in rows {
        writer.write_record(row)?;
    }

    writer.flush()?;
    Ok(())
}

/// Writes a delimited text file to CSV using optional header override.
pub fn write_lines_to_csv(
    lines: &[String],
    headers: Option<&[&str]>,
    input_delimiter: u8,
    output_path: &Path,
    flexible: bool,
) -> Result<(), IngestCsvError> {
    let rows: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            normalize_delimited_line(line, input_delimiter, false)
                .map_err(|err| IngestCsvError::new(format!("line normalization failed: {err}")))
        })
        .collect::<Result<Vec<_>, _>>()?;

    write_rows_to_csv(&rows, headers, output_path, flexible)
}

/// Writes typed records to CSV via serde serialization.
pub fn write_records_to_csv<T>(records: &[T], output_path: &Path) -> Result<(), IngestCsvError>
where
    T: Serialize,
{
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(output_path)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);

    for record in records {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}
