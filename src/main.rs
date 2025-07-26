use jiff::Timestamp;
use serde::Deserialize;
use std::process::Command;
use std::sync::Arc;
use std::time::Instant;
use trustfall::execute_query;

mod adapter;

pub mod podman {
    use super::*;

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
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ImageOutput {
    Podman(podman::Image),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Image {
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
            (img.id.clone(), String::new())
        };
        Self {
            repository,
            tag,
            size: img.size,
            created_at: img.created_at,
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::adapter::*;
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn can_deserialize_image() {
        let json = r#"
            {
                "Id": "e9d2252ab371a1149d3ef64b7793a274375dee5d9ec61b9e4fb41d75f156c1a1",
                "ParentId": "",
                "RepoTags": null,
                "RepoDigests": [
                    "docker.io/library/ubuntu@sha256:cdf755952ed117f6126ff4e65810bf93767d4c38f5c7185b50ec1f1078b464cc",
                    "docker.io/library/ubuntu@sha256:f995e05e8adc3292853cc37e6edda72351f8002ce7469a29322d19e01529cb9f"
                ],
                "Size": 82756709,
                "SharedSize": 0,
                "VirtualSize": 82756709,
                "Labels": {
                    "org.opencontainers.image.ref.name": "ubuntu",
                    "org.opencontainers.image.version": "24.10"
                },
                "Containers": 1,
                "Digest": "sha256:cdf755952ed117f6126ff4e65810bf93767d4c38f5c7185b50ec1f1078b464cc",
                "History": [
                    "docker.io/library/ubuntu:24.10"
                ],
                "Names": [
                    "docker.io/library/ubuntu:24.10"
                ],
                "Created": 1750414636,
                "CreatedAt": "2025-06-20T10:17:16Z"
            }"#;

        let _image: podman::Image = serde_json::from_str(json).unwrap();
        let image: ImageOutput = serde_json::from_str(json).unwrap();
        assert!(matches!(image, ImageOutput::Podman(_)));
    }

    #[test]
    fn get_june_images() {
        let query = r#"{
          Image {
            created_after(timestamp: "2025-06-01 00:00:00+00") {
              created_before(timestamp: "2025-07-01 00:00:00+00") {
                repo @output
                tag @output
                size @output
                created @output
              }
            }
          }
        }"#;

        let adapter = Arc::new(Adapter::new());
        let args: BTreeMap<Arc<str>, trustfall::FieldValue> = BTreeMap::new();

        let vertices = execute_query(Adapter::schema(), adapter, query, args).unwrap();
        println!("Printing vertices");
        for v in vertices {
            println!("{:?}", v);
        }
    }
}
