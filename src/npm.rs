use crate::model::Component;
use anyhow::Result;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_json(path: &Path) -> Result<Value> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let v: Value = serde_json::from_str(&buf)?;
    Ok(v)
}

fn make_purl(name: &str, version: &str) -> String {
    let enc_name = percent_encoding::utf8_percent_encode(name, percent_encoding::NON_ALPHANUMERIC)
        .to_string()
        .replace('%', "%25");
    format!("pkg:npm/{}@{}", enc_name.replace('@', "%40").replace('/', "%2F"), version)
}

fn extract_from_packages(obj: &Value, out: &mut Vec<Component>) {
    if let Some(map) = obj.as_object() {
        for (k, v) in map.iter() {
            if let Some(version) = v.get("version").and_then(|x| x.as_str()) {
                let name = if k.starts_with("node_modules/") {
                    k.strip_prefix("node_modules/").unwrap_or(k)
                } else {
                    k
                };
                let purl = make_purl(name, version);
                out.push(Component {
                    name: name.to_string(),
                    version: version.to_string(),
                    purl,
                    supplier: None,
                    licenses: None,
                });
            }
            if let Some(dep) = v.get("dependencies").and_then(|x| x.as_object()) {
                let mut tmp = Vec::new();
                extract_from_deps_obj(dep, &mut tmp);
                out.extend(tmp);
            }
        }
    }
}

fn extract_from_deps_obj(dep: &serde_json::Map<String, Value>, out: &mut Vec<Component>) {
    for (name, v) in dep.iter() {
        if let Some(version) = v.as_str() {
            let purl = make_purl(name, version);
            out.push(Component {
                name: name.clone(),
                version: version.to_string(),
                purl,
                supplier: None,
                licenses: None,
            });
        } else if let Some(obj) = v.as_object() {
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
        }
    }
}

pub fn parse_package_lock(path: &Path) -> Result<Vec<Component>> {
    let v = read_json(path)?;
    let mut out: Vec<Component> = Vec::new();
    if let Some(lock_ver) = v.get("lockfileVersion").and_then(|x| x.as_u64()) {
        if lock_ver == 2 || lock_ver == 3 {
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
    } else {
        if let Some(packages) = v.get("packages") {
            extract_from_packages(packages, &mut out);
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
