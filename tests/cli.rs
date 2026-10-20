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

/// npm v7+ entries carry a `dependencies` map whose values are *declared
/// semver ranges* (`{"debug": "^4.3.4"}`), not resolved packages. Those were
/// emitted as components with the range itself as the version, so a real
/// project produced one bogus duplicate per declared range
/// (`pkg:npm/debug@%5E4.3.4` alongside `pkg:npm/debug@4.3.4`).
#[test]
fn declared_dependency_ranges_are_not_components() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("package-lock.json");
    let mut f = File::create(&p).unwrap();
    f.write_all(
        br#"{
          "name": "app",
          "version": "1.0.0",
          "lockfileVersion": 3,
          "requires": true,
          "packages": {
            "": {
              "name": "app",
              "version": "1.0.0",
              "dependencies": { "debug": "^4.3.4" }
            },
            "node_modules/debug": {
              "version": "4.3.4",
              "resolved": "https://registry.npmjs.org/debug/-/debug-4.3.4.tgz",
              "dependencies": { "ms": "2.1.2" }
            },
            "node_modules/ms": {
              "version": "2.1.2",
              "resolved": "https://registry.npmjs.org/ms/-/ms-2.1.2.tgz"
            }
          }
        }"#,
    )
    .unwrap();

    let comps = generate_from_path(&p, Format::Cyclonedx).unwrap();
    let got = component_pairs(&comps);
    assert_eq!(
        got,
        vec![
            ("debug".to_string(), "4.3.4".to_string()),
            ("ms".to_string(), "2.1.2".to_string()),
        ],
        "expected exactly the resolved packages, no declared ranges: {got:?}"
    );
}

/// A nested key such as `node_modules/a/node_modules/@scope/deep` is the
/// package `@scope/deep` resolved against its parent `a`. Stripping only the
/// first `node_modules/` prefix produced names and purls that name no real
/// package (`pkg:npm/a/node_modules/@scope/deep@4.0.0`).
#[test]
fn nested_node_modules_paths_resolve_to_package_names() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("package-lock.json");
    let mut f = File::create(&p).unwrap();
    f.write_all(
        br#"{
          "name": "nested",
          "lockfileVersion": 3,
          "packages": {
            "": { "name": "nested", "version": "1.0.0" },
            "node_modules/@scope/pkg": { "version": "3.0.0" },
            "node_modules/a": { "version": "1.0.0" },
            "node_modules/a/node_modules/@scope/deep": { "version": "4.0.0" },
            "node_modules/a/node_modules/b": { "version": "2.5.1" }
          }
        }"#,
    )
    .unwrap();

    let comps = generate_from_path(&p, Format::Cyclonedx).unwrap();
    let mut got = component_pairs(&comps);
    got.sort();
    let mut want = vec![
        ("@scope/deep".to_string(), "4.0.0".to_string()),
        ("@scope/pkg".to_string(), "3.0.0".to_string()),
        ("a".to_string(), "1.0.0".to_string()),
        ("b".to_string(), "2.5.1".to_string()),
    ];
    want.sort();
    assert_eq!(got, want, "nested paths must yield plain package names");
}

/// `(name, version)` pairs from a generated CycloneDX document, sorted so
/// assertions do not depend on output ordering.
fn component_pairs(sbom: &str) -> Vec<(String, String)> {
    let doc: serde_json::Value = serde_json::from_str(sbom).expect("output must be valid JSON");
    let mut pairs: Vec<(String, String)> = doc["components"]
        .as_array()
        .expect("CycloneDX document must have components")
        .iter()
        .map(|c| {
            (
                c["name"].as_str().unwrap_or_default().to_string(),
                c["version"].as_str().unwrap_or_default().to_string(),
            )
        })
        .collect();
    pairs.sort();
    pairs
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
