use snapsbom::{generate_from_path, Format};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

fn write_sample(dir: &TempDir) -> PathBuf {
    let p = dir.path().join("package-lock.json");
    let mut f = File::create(&p).unwrap();
    f.write_all(br#"{
  "name": "demo",
  "lockfileVersion": 3,
  "packages": {
    "node_modules/a": {"version": "1.0.0"},
    "node_modules/b": {"version": "2.0.0"}
  }
}"#).unwrap();
    p
}

#[test]
fn test_deterministic_spdx() {
    let dir = TempDir::new().unwrap();
    let p = write_sample(&dir);
    let a = generate_from_path(&p, Format::Spdx).unwrap();
    let b = generate_from_path(&p, Format::Spdx).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_deterministic_cyclonedx() {
    let dir = TempDir::new().unwrap();
    let p = write_sample(&dir);
    let a = generate_from_path(&p, Format::Cyclonedx).unwrap();
    let b = generate_from_path(&p, Format::Cyclonedx).unwrap();
    assert_eq!(a, b);
}
