use crate::model::Component;
use serde_json::{json, Value};

fn spdx_version() -> &'static str {
    "SPDX-2.3"
}

fn data_license() -> &'static str {
    "CC0-1.0"
}

pub fn components_to_spdx(components: &[Component]) -> Value {
    let mut packages: Vec<Value> = Vec::new();
    for (i, c) in components.iter().enumerate() {
        let pkg_ref = format!("SPDXRef-Package-{}", i + 1);
        packages.push(json!({
            "SPDXID": pkg_ref,
            "copyrightText": "NOASSERTION",
            "downloadLocation": "NOASSERTION",
            "externalRefs": [{
                "referenceCategory": "PACKAGE-MANAGER",
                "referenceType": "purl",
                "referenceLocator": c.purl
            }],
            "filesAnalyzed": false,
            "licenseConcluded": "NOASSERTION",
            "licenseDeclared": "NOASSERTION",
            "name": c.name,
            "supplier": "NOASSERTION",
            "versionInfo": c.version
        }));
    }
    let mut rels: Vec<Value> = Vec::new();
    for (i, _c) in components.iter().enumerate() {
        let pkg_ref = format!("SPDXRef-Package-{}", i + 1);
        rels.push(json!({
            "spdxElementId": "SPDXRef-DOCUMENT",
            "relatedSpdxElement": pkg_ref,
            "relationshipType": "DESCRIBES"
        }));
    }
    json!({
        "SPDXID": "SPDXRef-DOCUMENT",
        "creationInfo": {
            "created": "2026-01-01T00:00:00Z",
            "creators": ["Tool: SnapSBOM"]
        },
        "dataLicense": data_license(),
        "documentNamespace": "https://spdx.org/spdxdocs/SPDXRef-DOCUMENT-STATIC",
        "name": "SnapSBOM SBOM",
        "packages": packages,
        "relationships": rels,
        "spdxVersion": spdx_version()
    })
}
