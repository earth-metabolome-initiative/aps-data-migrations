use std::{env, path::PathBuf};

fn main() {
    let output_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data/catalogue_of_life/base"));

    let dataset_id = env::args()
        .nth(2)
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(aps_data_migrations::catalogue_of_life::CATALOGUE_OF_LIFE_BASE_DATASET_ID);

    let summary = aps_data_migrations::catalogue_of_life::utils::download_col_base_and_export_csvs(
        &output_dir,
        dataset_id,
    )
    .unwrap_or_else(|err| {
        panic!(
            "failed to download/export CoL Base release to {}: {}",
            output_dir.display(),
            err
        )
    });

    println!(
        "Wrote {} CoL CSV files to {} (archive: {})",
        summary.written_csvs.len(),
        summary.normalized_dir.display(),
        summary.archive_path.display(),
    );
}
