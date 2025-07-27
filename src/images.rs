use crate::podman;
use jiff::Timestamp;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ImageOutput {
    Podman(podman::Image),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Image {
    pub hash: String,
    pub repository: String,
    pub tag: String,
    pub size: usize,
    pub created_at: Timestamp,
}

impl From<ImageOutput> for Image {
    fn from(x: ImageOutput) -> Self {
        match x {
            ImageOutput::Podman(p) => p.into(),
        }
    }
}

impl From<podman::Image> for Image {
    fn from(img: podman::Image) -> Self {
        let (repository, tag) = if let Some(s) = img.names.get(0) {
            let mut parts = s.split(":");
            let repo = parts.next().unwrap().to_string();
            let tag = parts.next().unwrap_or_else(|| "latest").to_string();
            (repo, tag)
        } else {
            (String::new(), String::new())
        };
        Self {
            hash: img.id,
            repository,
            tag,
            size: img.size,
            created_at: img.created_at,
        }
    }
}
