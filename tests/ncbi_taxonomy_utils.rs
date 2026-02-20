//! Integration tests for `ncbi_taxonomy::utils` download, deserialization, and CSV export helpers.

use std::{collections::HashMap, path::PathBuf};

use aps_data_migrations::ncbi_taxonomy::{
    CITATIONS_DMP_HEADERS, CitationsRecord, DELNODES_DMP_HEADERS, DIVISION_DMP_HEADERS,
    DelNodesRecord, DivisionRecord, EXCLUDED_FROM_TYPE_DMP_HEADERS, ExcludedFromTypeRecord,
    FULLNAMELINEAGE_DMP_HEADERS, FullNameLineageRecord, GENCODE_DMP_HEADERS, GenCodeRecord,
    HOST_DMP_HEADERS, HostRecord, IMAGES_DMP_HEADERS, ImagesRecord, MERGED_DMP_HEADERS,
    MergedRecord, NAMES_DMP_HEADERS, NEW_TAXDUMP_FILE_SPECS, NODES_DMP_HEADERS, NamesRecord,
    NodesRecord, RANKEDLINEAGE_DMP_HEADERS, RankedLineageRecord, TAXIDLINEAGE_DMP_HEADERS,
    TYPE_MATERIAL_DMP_HEADERS, TYPE_OF_TYPE_DMP_HEADERS, TaxIdLineageRecord, TypeMaterialRecord,
    TypeOfTypeRecord,
    utils::{
        deserialize_dmp_line, deserialize_dmp_lines, download_new_taxdump_and_write_csvs,
        download_new_taxdump_dmp_first_lines, write_dmp_lines_to_csv, write_records_to_csv,
    },
};

fn assert_first_row_deserializes<T>(
    rows: &HashMap<String, String>,
    file_name: &str,
    headers: &[&str],
) where
    T: for<'de> serde::Deserialize<'de>,
{
    let row = rows
        .get(file_name)
        .unwrap_or_else(|| panic!("missing first row for {file_name}"));

    let _: T = deserialize_dmp_line(row, headers)
        .unwrap_or_else(|err| panic!("failed to deserialize {file_name}: {err}"));
}

fn unique_test_path(file_name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock went backwards")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "aps_ncbi_taxonomy_{}_{}_{}",
        std::process::id(),
        nanos,
        file_name
    ))
}

#[test]
fn deserializes_first_row_of_each_new_taxdump_document() {
    let rows =
        download_new_taxdump_dmp_first_lines().expect("failed to download or unpack new_taxdump");

    for spec in NEW_TAXDUMP_FILE_SPECS {
        assert!(
            rows.contains_key(spec.file_name),
            "archive missing {}",
            spec.file_name
        );
        assert!(
            !spec.headers.is_empty(),
            "header spec for {} must not be empty",
            spec.file_name
        );
    }

    assert_first_row_deserializes::<CitationsRecord>(&rows, "citations.dmp", CITATIONS_DMP_HEADERS);
    assert_first_row_deserializes::<DelNodesRecord>(&rows, "delnodes.dmp", DELNODES_DMP_HEADERS);
    assert_first_row_deserializes::<DivisionRecord>(&rows, "division.dmp", DIVISION_DMP_HEADERS);
    assert_first_row_deserializes::<ExcludedFromTypeRecord>(
        &rows,
        "excludedfromtype.dmp",
        EXCLUDED_FROM_TYPE_DMP_HEADERS,
    );
    assert_first_row_deserializes::<FullNameLineageRecord>(
        &rows,
        "fullnamelineage.dmp",
        FULLNAMELINEAGE_DMP_HEADERS,
    );
    assert_first_row_deserializes::<GenCodeRecord>(&rows, "gencode.dmp", GENCODE_DMP_HEADERS);
    assert_first_row_deserializes::<HostRecord>(&rows, "host.dmp", HOST_DMP_HEADERS);
    assert_first_row_deserializes::<ImagesRecord>(&rows, "images.dmp", IMAGES_DMP_HEADERS);
    assert_first_row_deserializes::<MergedRecord>(&rows, "merged.dmp", MERGED_DMP_HEADERS);
    assert_first_row_deserializes::<NamesRecord>(&rows, "names.dmp", NAMES_DMP_HEADERS);
    assert_first_row_deserializes::<NodesRecord>(&rows, "nodes.dmp", NODES_DMP_HEADERS);
    assert_first_row_deserializes::<RankedLineageRecord>(
        &rows,
        "rankedlineage.dmp",
        RANKEDLINEAGE_DMP_HEADERS,
    );
    assert_first_row_deserializes::<TaxIdLineageRecord>(
        &rows,
        "taxidlineage.dmp",
        TAXIDLINEAGE_DMP_HEADERS,
    );
    assert_first_row_deserializes::<TypeMaterialRecord>(
        &rows,
        "typematerial.dmp",
        TYPE_MATERIAL_DMP_HEADERS,
    );
    assert_first_row_deserializes::<TypeOfTypeRecord>(
        &rows,
        "typeoftype.dmp",
        TYPE_OF_TYPE_DMP_HEADERS,
    );
}

#[test]
fn normalizes_and_writes_dmp_lines_to_csv() {
    let lines = vec!["1\t|\tall\t|\t\t|\tsynonym\t|\n".to_string()];
    let output_path = unique_test_path("names.csv");

    write_dmp_lines_to_csv(&lines, NAMES_DMP_HEADERS, &output_path)
        .expect("expected names rows to be written as csv");

    let written = std::fs::read_to_string(&output_path).expect("expected csv output file");
    assert!(written.starts_with("tax_id,name_txt,unique_name,name_class"));
    assert!(written.contains("1,all,,synonym"));

    let _ = std::fs::remove_file(output_path);
}

#[test]
fn deserializes_and_writes_typed_records_to_csv() {
    let lines = vec!["1\t|\tall\t|\t\t|\tsynonym\t|\n".to_string()];
    let records: Vec<NamesRecord> =
        deserialize_dmp_lines(&lines, NAMES_DMP_HEADERS).expect("expected typed deserialization");
    let output_path = unique_test_path("names_typed.csv");

    write_records_to_csv(&records, &output_path).expect("expected typed records to be written");

    let written = std::fs::read_to_string(&output_path).expect("expected typed csv output file");
    assert!(written.starts_with("tax_id,name_txt,unique_name,name_class"));
    assert!(written.contains("1,all,,synonym"));

    let _ = std::fs::remove_file(output_path);
}

#[test]
fn download_and_write_csvs_creates_files_for_all_specs() {
    let output_dir = unique_test_path("artifact_csvs");

    let written_paths = download_new_taxdump_and_write_csvs(&output_dir)
        .expect("expected csv export for all known dmp artifacts");

    assert_eq!(written_paths.len(), NEW_TAXDUMP_FILE_SPECS.len());
    for path in written_paths {
        assert!(
            path.exists(),
            "expected output csv to exist: {}",
            path.display()
        );
    }

    let _ = std::fs::remove_dir_all(output_dir);
}
