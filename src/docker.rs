use jiff::Zoned;
use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    pub created_at: Zoned,
    #[serde(alias = "ID")]
    pub id: String,
    pub repository: Option<String>,
    pub size: human_size::Size,
    pub tag: Option<String>,
}
