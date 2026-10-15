use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use console_error_panic_hook;
use crate::{det, spdx, cyclonedx, model::Component};

#[wasm_bindgen]
pub fn init() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn gen_spdx_from_json(json: &str) -> Result<String, JsValue> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let comps = parse_lock(v)?;
    let out = spdx::components_to_spdx(&comps);
    det::to_deterministic_json(&out).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn gen_cyclonedx_from_json(json: &str) -> Result<String, JsValue> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let comps = parse_lock(v)?;
    let out = cyclonedx::components_to_cyclonedx(&comps);
    det::to_deterministic_json(&out).map_err(|e| JsValue::from_str(&e.to_string()))
}

fn parse_lock(v: serde_json::Value) -> Result<Vec<Component>, JsValue> {
    let mut out: Vec<Component> = Vec::new();
    if let Some(packages) = v.get("packages").and_then(|x| x.as_object()) {
        for (k, pv) in packages.iter() {
            if let Some(ver) = pv.get("version").and_then(|x| x.as_str()) {
                let name = if k.starts_with("node_modules/") { k.strip_prefix("node_modules/").unwrap_or(k) } else { k };
                if name.is_empty() { continue; }
                let purl = crate::npm::make_purl(name, ver);
                out.push(Component { name: name.to_string(), version: ver.to_string(), purl, supplier: None, licenses: None });
            }
        }
    }
    out.sort_unstable();
    let mut seen = std::collections::BTreeMap::new();
    for c in out {
        seen.entry(c.purl.clone()).or_insert(c);
    }
    Ok(seen.into_values().collect())
}
