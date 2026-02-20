//! Integration tests for Catalogue of Life Base release utilities.

use aps_data_migrations::catalogue_of_life::{
    CATALOGUE_OF_LIFE_BASE_DATASET_ID, DistributionRecord, MediaRecord, NameRelationRecord,
    NameUsageRecord, ReferenceRecord, SpeciesEstimateRecord, SpeciesInteractionRecord,
    TaxonConceptRelationRecord, TaxonPropertyRecord, TypeMaterialRecord, VernacularNameRecord,
    utils::{
        col_base_export_url, deserialize_col_tsv_line, download_col_base_and_export_csvs,
        download_col_base_tabular_lines,
    },
};
use std::path::PathBuf;

fn unique_path(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock went backwards")
        .as_nanos();
    std::env::temp_dir().join(format!("aps_col_{}_{}_{}", std::process::id(), nanos, name))
}

#[test]
fn builds_base_export_url() {
    let url = col_base_export_url(CATALOGUE_OF_LIFE_BASE_DATASET_ID, true);
    assert!(url.contains("/dataset/"));
    assert!(url.contains("export.zip"));
    assert!(url.contains("format=ColDP"));
}

fn find_rows_for_file<'a>(
    rows_by_file: &'a std::collections::HashMap<String, Vec<String>>,
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

fn deserialize_first_data_row<T>(rows: &[String])
where
    T: for<'de> serde::Deserialize<'de>,
{
    if rows.len() < 2 {
        return;
    }
    let headers_owned: Vec<String> = rows[0]
        .trim_end_matches('\n')
        .split('\t')
        .map(|s| s.to_string())
        .collect();
    let headers: Vec<&str> = headers_owned.iter().map(String::as_str).collect();
    let _: T = deserialize_col_tsv_line(&rows[1], &headers).expect("deserialize first row");
}

#[test]
#[ignore = "live network download"]
fn exports_base_release_to_raw_and_normalized_layout() {
    let out = unique_path("base_export");
    let summary = download_col_base_and_export_csvs(&out, CATALOGUE_OF_LIFE_BASE_DATASET_ID)
        .expect("export base release");

    assert!(summary.archive_path.exists());
    assert!(summary.extracted_dir.exists());
    assert!(summary.normalized_dir.exists());
    assert!(!summary.written_csvs.is_empty());

    let _ = std::fs::remove_dir_all(out);
}

#[test]
#[ignore = "live network download"]
fn deserializes_first_rows_into_col_structs() {
    let rows_by_file = download_col_base_tabular_lines(CATALOGUE_OF_LIFE_BASE_DATASET_ID)
        .expect("download CoL rows");

    deserialize_first_data_row::<DistributionRecord>(
        find_rows_for_file(&rows_by_file, "Distribution.tsv").expect("distribution table"),
    );
    deserialize_first_data_row::<MediaRecord>(
        find_rows_for_file(&rows_by_file, "Media.tsv").expect("media table"),
    );
    deserialize_first_data_row::<NameRelationRecord>(
        find_rows_for_file(&rows_by_file, "NameRelation.tsv").expect("name relation table"),
    );
    deserialize_first_data_row::<NameUsageRecord>(
        find_rows_for_file(&rows_by_file, "NameUsage.tsv").expect("name usage table"),
    );
    deserialize_first_data_row::<ReferenceRecord>(
        find_rows_for_file(&rows_by_file, "Reference.tsv").expect("reference table"),
    );
    deserialize_first_data_row::<SpeciesEstimateRecord>(
        find_rows_for_file(&rows_by_file, "SpeciesEstimate.tsv").expect("species estimate table"),
    );
    deserialize_first_data_row::<SpeciesInteractionRecord>(
        find_rows_for_file(&rows_by_file, "SpeciesInteraction.tsv")
            .expect("species interaction table"),
    );
    deserialize_first_data_row::<TaxonConceptRelationRecord>(
        find_rows_for_file(&rows_by_file, "TaxonConceptRelation.tsv")
            .expect("taxon concept relation table"),
    );
    deserialize_first_data_row::<TaxonPropertyRecord>(
        find_rows_for_file(&rows_by_file, "TaxonProperty.tsv").expect("taxon property table"),
    );
    deserialize_first_data_row::<TypeMaterialRecord>(
        find_rows_for_file(&rows_by_file, "TypeMaterial.tsv").expect("type material table"),
    );
    deserialize_first_data_row::<VernacularNameRecord>(
        find_rows_for_file(&rows_by_file, "VernacularName.tsv").expect("vernacular name table"),
    );
}
