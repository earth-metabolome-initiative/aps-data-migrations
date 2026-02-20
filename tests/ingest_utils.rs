//! Integration tests for shared `ingest` helpers.

use aps_data_migrations::ingest::{
    archive::{extract_zip_to_dir, read_tar_gz_text_files, read_zip_text_files},
    csv::{normalize_delimited_line, write_lines_to_csv},
};
use flate2::{Compression, write::GzEncoder};
use std::{io::Write, path::PathBuf};
use tar::Builder;
use zip::write::FileOptions;

fn unique_path(name: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock went backwards")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "aps_ingest_{}_{}_{}",
        std::process::id(),
        nanos,
        name
    ))
}

#[test]
fn normalizes_delimited_lines() {
    let cols = normalize_delimited_line("a\tb\tc\n", b'\t', false).expect("normalize");
    assert_eq!(cols, vec!["a", "b", "c"]);
}

#[test]
fn writes_flexible_csv_from_lines() {
    let lines = vec!["a\tb\n".to_string(), "a\tb\tc\n".to_string()];
    let out = unique_path("flex.csv");

    write_lines_to_csv(&lines, None, b'\t', &out, true).expect("write flexible csv");
    let written = std::fs::read_to_string(&out).expect("read written csv");
    assert!(written.contains("a,b"));
    assert!(written.contains("a,b,c"));
    let _ = std::fs::remove_file(out);
}

#[test]
fn reads_zip_text_files() {
    let mut cursor = std::io::Cursor::new(Vec::<u8>::new());
    {
        let mut zip = zip::ZipWriter::new(&mut cursor);
        let opts: FileOptions<'_, ()> = FileOptions::default();
        zip.start_file("Taxon.tsv", opts).expect("start file");
        zip.write_all(b"id\tname\n1\tLife\n").expect("write data");
        zip.finish().expect("finish zip");
    }

    let rows = read_zip_text_files(cursor.get_ref(), &|name| name.ends_with(".tsv"))
        .expect("read zip rows");
    assert!(rows.contains_key("Taxon.tsv"));
}

#[test]
fn reads_tar_gz_text_files() {
    let mut tar_bytes = Vec::new();
    {
        let enc = GzEncoder::new(&mut tar_bytes, Compression::default());
        let mut tar = Builder::new(enc);
        let mut header = tar::Header::new_gnu();
        let content = b"1\t|\n";
        header.set_size(content.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "sample.dmp", &content[..])
            .expect("append tar data");
        tar.finish().expect("finish tar");
    }

    let rows = read_tar_gz_text_files(&tar_bytes, &|name| name.ends_with(".dmp"))
        .expect("read tar.gz rows");
    assert!(rows.contains_key("sample.dmp"));
}

#[test]
fn extracts_zip_to_directory() {
    let mut cursor = std::io::Cursor::new(Vec::<u8>::new());
    {
        let mut zip = zip::ZipWriter::new(&mut cursor);
        let opts: FileOptions<'_, ()> = FileOptions::default();
        zip.start_file("nested/NameUsage.tsv", opts)
            .expect("start file");
        zip.write_all(b"id\tname\n").expect("write data");
        zip.finish().expect("finish zip");
    }

    let out = unique_path("zip_extract");
    let files = extract_zip_to_dir(cursor.get_ref(), &out).expect("extract zip");
    assert!(!files.is_empty());
    assert!(out.join("nested/NameUsage.tsv").exists());

    let _ = std::fs::remove_dir_all(out);
}
