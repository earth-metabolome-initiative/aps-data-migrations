//! Progress bar helpers.

use indicatif::ProgressStyle;

/// Builds a progress style from template, falling back to default style.
pub fn progress_style(template: &str) -> ProgressStyle {
    ProgressStyle::with_template(template).unwrap_or_else(|_| ProgressStyle::default_bar())
}
