use jiff::Timestamp;
use serde::Deserialize;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    pub id: String,
    pub parent_id: String,
    pub size: usize,
    pub history: Vec<String>,
    #[serde(default)]
    pub names: Vec<String>,
    pub created: usize,
    pub created_at: Timestamp,
}
