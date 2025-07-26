use crate::ImageOutput;
use std::process::Command;
use std::sync::Arc;
use trustfall::provider::{ResolveInfo, VertexIterator};

use super::vertex::Vertex;

pub(super) fn image<'a>(_resolve_info: &ResolveInfo) -> VertexIterator<'a, Vertex> {
    let images = Command::new("docker")
        .args(["image", "ls", "--format", "json"])
        .output()
        .expect("failed to run docker");

    let images: Vec<ImageOutput> =
        serde_json::from_slice(&images.stdout).expect("couldn't deserialize the json output");

    Box::new(
        images
            .into_iter()
            .map(|x| Vertex::Image(Arc::new(x.into()))),
    )
}
