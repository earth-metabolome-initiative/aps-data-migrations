use std::{env, path::PathBuf};

fn main() {
    let output_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data/ncbi_taxonomy_csvs"));

    let written_paths =
        aps_data_migrations::ncbi_taxonomy::utils::download_new_taxdump_and_write_csvs(&output_dir)
            .unwrap_or_else(|err| {
                panic!(
                    "failed to download and export NCBI taxonomy CSVs to {}: {}",
                    output_dir.display(),
                    err
                )
            });

    println!(
        "Wrote {} CSV files to {}",
        written_paths.len(),
        output_dir.display()
    );
}
