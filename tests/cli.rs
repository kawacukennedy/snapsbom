use assert_cmd::Command;
use predicates::prelude::*;
use snapsbom::{generate_from_path, Format};
use std::fs::{create_dir, File};
use std::io::Write;
use tempfile::TempDir;

#[test]
fn cli_detects_package_lock() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("package-lock.json");
    let mut f = File::create(&p).unwrap();
    f.write_all(br#"{"lockfileVersion": 3, "packages": {"node_modules/x": {"version": "1.0.0"}}}"#).unwrap();
    let mut cmd = Command::cargo_bin("snapsbom").unwrap();
    cmd.arg(dir.path());
    cmd.assert().success().stdout(predicate::str::contains("SPDXRef-Package"));
}

/// A file that is not a lockfile must fail loudly rather than emit an
/// empty SBOM with exit code 0.
#[test]
fn rejects_non_lockfile_json() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("data.json");
    let mut f = File::create(&p).unwrap();
    f.write_all(br#"{"hello": "world"}"#).unwrap();
    let mut cmd = Command::cargo_bin("snapsbom").unwrap();
    cmd.arg(&p);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("does not look like an npm package-lock.json"));
}

/// `package.json` has `dependencies`, but only version *ranges*. Treating it
/// as a lockfile would invent purls like `pkg:npm/foo@^1.2.3`.
#[test]
fn rejects_package_json() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("package.json");
    let mut f = File::create(&p).unwrap();
    f.write_all(br#"{"name": "x", "dependencies": {"chalk": "^5.4.1"}}"#).unwrap();
    let mut cmd = Command::cargo_bin("snapsbom").unwrap();
    cmd.arg(&p);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("does not look like an npm package-lock.json"));
}

#[test]
fn directory_without_lockfile_errors_clearly() {
    let dir = TempDir::new().unwrap();
    let empty = dir.path().join("sub");
    create_dir(&empty).unwrap();
    let mut cmd = Command::cargo_bin("snapsbom").unwrap();
    cmd.arg(&empty);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("no package-lock.json found in"));
}

#[test]
fn missing_path_errors_clearly() {
    let mut cmd = Command::cargo_bin("snapsbom").unwrap();
    cmd.arg("/definitely/not/a/real/path.json");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

/// npm v5/v6 lockfiles (`lockfileVersion: 1`) used to parse to zero
/// packages because only versions 2 and 3 were handled.
#[test]
fn parses_lockfile_version_1_with_nested_deps() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("package-lock.json");
    let mut f = File::create(&p).unwrap();
    f.write_all(
        br#"{
          "name": "legacy",
          "lockfileVersion": 1,
          "dependencies": {
            "a": {
              "version": "1.0.0",
              "dependencies": { "nested": { "version": "9.9.9" } }
            },
            "b": { "version": "2.0.0" }
          }
        }"#,
    )
    .unwrap();

    let comps = generate_from_path(&p, Format::Spdx).unwrap();
    assert!(comps.contains("pkg:npm/a@1.0.0"), "got: {comps}");
    assert!(comps.contains("pkg:npm/b@2.0.0"), "got: {comps}");
    assert!(comps.contains("pkg:npm/nested@9.9.9"), "got: {comps}");
}
