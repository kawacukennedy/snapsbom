use crate::model::Component;
use serde_json::{json, Value};

pub fn components_to_cyclonedx(components: &[Component]) -> Value {
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
    json!({
        "bomFormat": "CycloneDX",
        "specVersion": "1.6",
        "version": 1,
        "metadata": {
            "timestamp": "2026-01-01T00:00:00Z",
            "tools": [{"name": "SnapSBOM", "version": env!("CARGO_PKG_VERSION")}],
            "component": {"type": "application", "name": "snapSBOM", "version": env!("CARGO_PKG_VERSION")}
        },
        "components": comps
    })
}
