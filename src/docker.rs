use jiff::{Timestamp, fmt::strtime::BrokenDownTime};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    #[serde(deserialize_with = "deserialize_docker_timestamp")]
    pub created_at: Timestamp,
    #[serde(alias = "ID")]
    pub id: String,
    pub repository: Option<String>,
    pub size: human_size::Size,
    pub tag: Option<String>,
}

fn deserialize_docker_timestamp<'de, D>(d: D) -> Result<Timestamp, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    let (tm, _) = BrokenDownTime::parse_prefix("%Y-%m-%d %H:%M:%S %z", &s)
        .map_err(serde::de::Error::custom)?;

    tm.to_timestamp().map_err(serde::de::Error::custom)
}
