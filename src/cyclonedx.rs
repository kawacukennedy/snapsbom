use crate::det;
use crate::model::Component;
use serde_json::{json, Value};

pub fn components_to_cyclonedx(components: &[Component]) -> Value {
    components_to_cyclonedx_opts(components, None)
}

/// Like [`components_to_cyclonedx`], but optionally emits a CycloneDX
/// `serialNumber`.
///
/// Omitting it (the default) keeps output byte-identical across runs. Passing
/// `Some(...)` opts into the CycloneDX "a new serialNumber per generation"
/// guidance, which is inherently non-deterministic — callers that want
/// reproducible output should leave this as `None`.
pub fn components_to_cyclonedx_opts(components: &[Component], serial_number: Option<String>) -> Value {
    let mut comps: Vec<Value> = Vec::new();
    for c in components.iter() {
        comps.push(json!({
            "type": "library",
            "name": c.name,
            "version": c.version,
            "purl": c.purl,
            "licenses": [],
            "supplier": {"name": "NOASSERTION"}
        }));
    }
    let mut doc = json!({
        "bomFormat": "CycloneDX",
        "specVersion": "1.6",
        "version": 1,
        "metadata": {
            "timestamp": det::deterministic_timestamp(),
            "tools": [{"name": "SnapSBOM", "version": env!("CARGO_PKG_VERSION")}],
            "component": {"type": "application", "name": "snapSBOM", "version": env!("CARGO_PKG_VERSION")}
        },
        "components": comps
    });
    if let Some(sn) = serial_number {
        if let Some(obj) = doc.as_object_mut() {
            obj.insert("serialNumber".to_string(), Value::String(sn));
        }
    }
    doc
}
