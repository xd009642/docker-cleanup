use adapter::*;
use args::Commands;
use clap::Parser;
use jiff::Timestamp;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::process::Command;
use std::sync::Arc;
use human_size::{SpecificSize, multiples::*};
use trustfall::execute_query;

mod adapter;
mod args;
pub mod images;
pub mod podman;

pub use images::*;

fn main() {
    let args = args::Cli::parse();

    println!("{:?}", args);

    let filter = args.command.filter();

    let mut closing_brackets = 0;
    let mut query_str = "{Image{".to_string();

    if let Some(created_before) = filter.created_before {
        query_str.push_str(&format!(
            "created_before(timestamp: \"{}\") {{",
            created_before
        ));
        closing_brackets += 1;
    }

    if let Some(created_before) = filter.created_after {
        query_str.push_str(&format!(
            "created_after(timestamp: \"{}\") {{",
            created_before
        ));
        closing_brackets += 1;
    }

    match (filter.smaller_than, filter.larger_than) {
        (Some(lt), Some(gt)) => {
            query_str.push_str(&format!("size_in_range(max: {}, min: {}) {{", lt, gt));
            closing_brackets += 1;
        }
        (Some(lt), None) => {
            query_str.push_str(&format!("size_in_range(max: {}, min: 0) {{", lt));
            closing_brackets += 1;
        }
        (None, Some(gt)) => {
            query_str.push_str(&format!("size_in_range(max: {}, min: {}) {{", i64::MAX, gt));
            closing_brackets += 1;
        }
        (None, None) => {}
    }

    query_str.push_str("repo @output\ntag @output\nsize @output\ncreated @output\n");

    for _ in 0..closing_brackets {
        query_str.push('}');
    }

    query_str.push_str("}}");
    let adapter = Arc::new(Adapter::new());
    let args: BTreeMap<Arc<str>, trustfall::FieldValue> = BTreeMap::new();

    let vertices = execute_query(Adapter::schema(), adapter, &query_str, args).unwrap();
    let images = vertices.map(|x| (format!("{}:{}", x["repo"].as_str().unwrap(), x["tag"].as_str().unwrap()), x["size"].as_u64().unwrap())).collect::<Vec<_>>();

    for (image, size) in &images{
        let human_size = SpecificSize::new(*size as f64, Byte).unwrap();
        let size = if *size > 1_000_000_000 {
            let s: SpecificSize::<Gigabyte> = human_size.into();
            s.to_string()
        } else {
            let s: SpecificSize::<Megabyte> = human_size.into();
            s.to_string()
        };
        println!("{}\t{}", image, size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize_podman_image() {
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
    #[ignore]
    fn can_deserialize_docker_image() {
        let json = r#"{"Containers":"N/A","CreatedAt":"2022-10-25 02:53:28 +0100 BST","CreatedSince":"2 years ago","Digest":"\u003cnone\u003e","ID":"71eaf13299f4","Repository":"ubuntu","SharedSize":"N/A","Size":"63.1MB","Tag":"18.04","UniqueSize":"N/A","VirtualSize":"63.15MB"}"#;

        let image: ImageOutput = serde_json::from_str(json).unwrap();
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
