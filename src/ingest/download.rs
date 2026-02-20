//! Shared HTTP download helpers.

use std::{error::Error, fmt, io::Read};

use indicatif::ProgressBar;
use reqwest::blocking::get;

use crate::ingest::progress::progress_style;

/// Error returned when downloading an archive or endpoint payload fails.
#[derive(Debug)]
pub struct IngestDownloadError {
    message: String,
}

impl IngestDownloadError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for IngestDownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for IngestDownloadError {}

impl From<reqwest::Error> for IngestDownloadError {
    fn from(value: reqwest::Error) -> Self {
        Self::new(format!("HTTP download error: {value}"))
    }
}

impl From<std::io::Error> for IngestDownloadError {
    fn from(value: std::io::Error) -> Self {
        Self::new(format!("Download I/O error: {value}"))
    }
}

/// Downloads bytes from URL with a progress indicator.
pub fn download_bytes(url: &str, label: &str) -> Result<Vec<u8>, IngestDownloadError> {
    let mut response = get(url)?.error_for_status()?;
    let total_size = response.content_length();

    let progress = match total_size {
        Some(total) => {
            let bar = ProgressBar::new(total);
            bar.set_style(progress_style(
                "{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
            ));
            bar
        }
        None => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(progress_style("{msg} {spinner} {bytes}"));
            spinner.enable_steady_tick(std::time::Duration::from_millis(100));
            spinner
        }
    };

    progress.set_message(format!("Downloading {label}"));

    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = response.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..read]);
        progress.inc(read as u64);
    }

    progress.finish_with_message(format!("Downloaded {label}"));
    Ok(bytes)
}
