use crate::model::Component;
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_json(path: &Path) -> Result<Value> {
    let mut file = File::open(path).with_context(|| format!("cannot open {}", path.display()))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let v: Value = serde_json::from_str(&buf)
        .with_context(|| format!("{} is not valid JSON", path.display()))?;
    Ok(v)
}

/// Characters that are safe to leave unescaped in a `pkg:npm` purl segment.
///
/// The purl spec only requires percent-encoding of characters that are not
/// unreserved (`A-Z a-z 0-9 - . _ ~`). In particular a hyphen must stay
/// literal, and the `/` that separates an npm scope from its package name is
/// preserved as a namespace separator (`@scope/name` -> `%40scope/name`).
const PURL_ESCAPE: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~')
    .remove(b'/');

pub(crate) fn make_purl(name: &str, version: &str) -> String {
    let enc_name = percent_encoding::utf8_percent_encode(name, PURL_ESCAPE);
    let enc_version = percent_encoding::utf8_percent_encode(version, PURL_ESCAPE);
    format!("pkg:npm/{}@{}", enc_name, enc_version)
}

fn extract_from_packages(obj: &Value, out: &mut Vec<Component>) {
    if let Some(map) = obj.as_object() {
        for (k, v) in map.iter() {
            if let Some(version) = v.get("version").and_then(|x| x.as_str()) {
                // The name is the segment after the *last* `node_modules/`:
                // `node_modules/a/node_modules/@scope/b` is the package
                // `@scope/b`, resolved against its parent. Stripping only
                // the first prefix produced purls like
                // `pkg:npm/a/node_modules/b@2.5.1`.
                let name = k.rsplit("node_modules/").next().unwrap_or(k);
                // Skip the root project entry (key `""`), which carries no
                // package name; only its dependencies belong in the SBOM.
                if !name.is_empty() {
                    let purl = make_purl(name, version);
                    out.push(Component {
                        name: name.to_string(),
                        version: version.to_string(),
                        purl,
                        supplier: None,
                        licenses: None,
                    });
                }
            }
            // npm v7+ entries also carry a `dependencies` object, but its
            // values are *declared semver ranges* (`{"ms": "^2.0.0"}`), not
            // resolved packages - and every resolved package is already a
            // key of this very map. Reading it here used to add one
            // duplicate component per declared range, with the range itself
            // used as the version (`pkg:npm/ms@%5E2.0.0`).
        }
    }
}

fn extract_from_deps_obj(dep: &serde_json::Map<String, Value>, out: &mut Vec<Component>) {
    for (name, v) in dep.iter() {
        // A string value is a *declared range*, never a resolved version -
        // both `package.json` and npm v7+ `packages.*.dependencies` look
        // like this (`{"chalk": "^5.4.1"}`). Emitting it would invent a
        // component that may not be installed at all, which is exactly how
        // `pkg:npm/chalk@%5E5.4.1` ended up in generated SBOMs.
        if let Some(obj) = v.as_object() {
            if let Some(version) = obj.get("version").and_then(|x| x.as_str()) {
                let purl = make_purl(name, version);
                out.push(Component {
                    name: name.clone(),
                    version: version.to_string(),
                    purl,
                    supplier: None,
                    licenses: None,
                });
            }
            // npm v5/v6 (`lockfileVersion: 1`) nests transitive dependencies
            // under each entry; walk them so nothing is silently dropped.
            if let Some(nested) = obj.get("dependencies").and_then(|x| x.as_object()) {
                extract_from_deps_obj(nested, out);
            }
        }
    }
}

pub fn parse_package_lock(path: &Path) -> Result<Vec<Component>> {
    let v = read_json(path)?;

    // Refuse input we don't recognise. Producing an empty SBOM for a file
    // that isn't a lockfile would be far worse than failing: a CI job would
    // report success while the generated SBOM listed zero packages. Note
    // that `package.json` is deliberately NOT accepted here - it has
    // `dependencies` but only version *ranges*, not resolved versions.
    let looks_like_lockfile = v.get("lockfileVersion").is_some()
        || v.get("packages").map(|p| p.is_object()).unwrap_or(false);
    if !looks_like_lockfile {
        anyhow::bail!(
            "{} does not look like an npm package-lock.json \
             (missing the \"lockfileVersion\" or \"packages\" key). \
             Pass the lockfile itself, or a directory containing one.",
            path.display()
        );
    }

    let mut out: Vec<Component> = Vec::new();
    match v.get("lockfileVersion").and_then(|x| x.as_u64()) {
        // npm v5/v6: everything lives in a `dependencies` tree.
        Some(1) => {
            if let Some(deps) = v.get("dependencies").and_then(|x| x.as_object()) {
                extract_from_deps_obj(deps, &mut out);
            }
        }
        // npm v7+: a flat `packages` map, with a `dependencies` fallback for
        // unusually small or hand-trimmed files.
        Some(_) => {
            if let Some(packages) = v.get("packages") {
                extract_from_packages(packages, &mut out);
            }
            if out.is_empty() {
                if let Some(deps) = v.get("dependencies") {
                    if let Some(m) = deps.as_object() {
                        extract_from_deps_obj(m, &mut out);
                    }
                }
            }
        }
        None => {
            if let Some(packages) = v.get("packages") {
                extract_from_packages(packages, &mut out);
            }
        }
    }
    let mut seen: BTreeMap<String, Component> = BTreeMap::new();
    for c in out {
        seen.entry(c.purl.clone()).or_insert(c);
    }
    let mut res: Vec<Component> = seen.into_values().collect();
    res.sort_unstable();
    Ok(res)
}
