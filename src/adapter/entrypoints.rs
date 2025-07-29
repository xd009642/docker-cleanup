use crate::ImageOutput;
use std::process::Command;
use std::sync::Arc;
use trustfall::provider::{ResolveInfo, VertexIterator};

use super::vertex::Vertex;

pub(super) fn image<'a>(_resolve_info: &ResolveInfo) -> VertexIterator<'a, Vertex> {
    let version = Command::new("docker")
        .args(["--version"])
        .output()
        .expect("couldn't get docker version");
    let version = String::from_utf8_lossy(&version.stdout);
    let is_podman = version.contains("podman");

    let images = Command::new("docker")
        .args(["image", "ls", "--format", "json"])
        .output()
        .expect("failed to run docker");

    let images: Vec<ImageOutput> = if is_podman {
        serde_json::from_slice(&images.stdout).expect("couldn't deserialize the json output")
    } else {
        let s = String::from_utf8_lossy(&images.stdout);
        let mut v = vec![];
        for line in s.lines() {
            v.push(serde_json::from_str(line).expect("couldn't deserialize the json output"));
        }
        v
    };

    Box::new(
        images
            .into_iter()
            .map(|x| Vertex::Image(Arc::new(x.into()))),
    )
}
