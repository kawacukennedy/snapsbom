use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub version: String,
    pub purl: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<String>>,
}

impl Component {
    pub fn key(&self) -> String {
        self.purl.clone()
    }
}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        self.purl == other.purl
    }
}

impl Eq for Component {}

impl PartialOrd for Component {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Component {
    fn cmp(&self, other: &Self) -> Ordering {
        self.purl
            .cmp(&other.purl)
            .then(self.name.cmp(&other.name))
            .then(self.version.cmp(&other.version))
    }
}
