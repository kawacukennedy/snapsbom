use std::fs;
use std::path::Path;

/// The crate version is embedded in generated SBOMs (`metadata.tools`) and is
/// what `snapsbom --version` prints, while the npm packages carry their own
/// copies. If they drift, `npx snapsbom` can resolve a launcher whose
/// `optionalDependencies` point at platform packages that were never
/// published - a broken install that only shows up on a fresh machine.
///
/// There are 7 places to keep in sync (`Cargo.toml`, the root `package.json`,
/// and one per platform package), so the test compares every npm reference
/// against `CARGO_PKG_VERSION` instead of trusting a manual bump.
#[test]
fn npm_metadata_matches_crate_version() {
    let expected = env!("CARGO_PKG_VERSION");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let read = |rel: &str| -> serde_json::Value {
        let path = root.join(rel);
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        serde_json::from_str(&text)
            .unwrap_or_else(|e| panic!("{} is not valid JSON: {e}", path.display()))
    };

    let main = read("package.json");
    assert_eq!(
        main["version"].as_str(),
        Some(expected),
        "package.json version drifted from Cargo.toml"
    );

    let optional = main["optionalDependencies"]
        .as_object()
        .expect("optionalDependencies must be an object");

    assert!(
        !optional.is_empty(),
        "the launcher resolves platform packages through optionalDependencies"
    );

    for (name, pinned) in optional {
        assert_eq!(
            pinned.as_str(),
            Some(expected),
            "package.json pins {name}@{pinned}, but the crate is {expected}"
        );

        let platform = read(&format!("npm/{name}/package.json"));
        assert_eq!(
            platform["name"].as_str(),
            Some(name.as_str()),
            "npm/{name}/package.json declares the wrong package name"
        );
        assert_eq!(
            platform["version"].as_str(),
            Some(expected),
            "npm/{name}/package.json version drifted from Cargo.toml"
        );
    }

    // Every platform package must be reachable from the launcher's pin list,
    // otherwise `npx snapsbom` would resolve a package nobody publishes.
    let entries = fs::read_dir(root.join("npm"))
        .expect("npm/ directory must exist")
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().to_str().map(str::to_string))
        .collect::<Vec<_>>();
    for entry in entries {
        assert!(
            optional.contains_key(&entry),
            "npm/{entry} exists but is not listed in optionalDependencies"
        );
    }
}
