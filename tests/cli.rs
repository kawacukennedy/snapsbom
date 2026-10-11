use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
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
