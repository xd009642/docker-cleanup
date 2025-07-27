use super::vertex::Vertex;
use regex::Regex;
use std::sync::Arc;
use trustfall::provider::{
    AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo,
    VertexIterator,
};

pub(super) fn resolve_image_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "created_after" => {
            let timestamp: &str = parameters
                .get("timestamp")
                .expect(
                    "failed to find parameter 'timestamp' for edge 'created_after' on type 'Image'",
                )
                .as_str()
                .expect("unexpected null or other incorrect datatype for Trustfall type 'String!'");
            image::created_after(contexts, timestamp, resolve_info)
        }
        "created_before" => {
            let timestamp: &str = parameters
                .get("timestamp")
                .expect(
                    "failed to find parameter 'timestamp' for edge 'created_before' on type 'Image'",
                )
                .as_str()
                .expect(
                    "unexpected null or other incorrect datatype for Trustfall type 'String!'",
                );
            image::created_before(contexts, timestamp, resolve_info)
        }
        "has_name" => {
            let name: Arc<str> = parameters
                .get("name")
                .expect("failed to find parameter 'name' for edge 'has_name' on type 'Image'")
                .as_arc_str()
                .expect("unexpected null or other incorrect datatype for Trustfall type 'String!'")
                .clone();
            image::has_name(contexts, name, resolve_info)
        }
        "size_in_range" => {
            let min: i64 = parameters
                .get("min")
                .expect("failed to find parameter 'min' for edge 'size_in_range' on type 'Image'")
                .as_i64()
                .expect("unexpected null or other incorrect datatype for Trustfall type 'Int!'");
            let max: i64 = parameters
                .get("max")
                .expect("failed to find parameter 'max' for edge 'size_in_range' on type 'Image'")
                .as_i64()
                .expect("unexpected null or other incorrect datatype for Trustfall type 'Int!'");
            image::size_in_range(contexts, min, max, resolve_info)
        }
        "name_matches" => {
            let regex: &str = parameters
                .get("regex")
                .expect("failed to find parameter 'regex' for edge 'name_matches' on type 'Image'")
                .as_str()
                .expect("unexpected null or other incorrect datatype for Trustfall type 'String!'");

            let regex = Regex::new(regex).unwrap();
            image::name_matches(contexts, regex, resolve_info)
        }
        "name_contains" => {
            let substring: Arc<str> = parameters
                .get("substring")
                .expect("failed to find parameter 'regex' for edge 'name_matches' on type 'Image'")
                .as_arc_str()
                .expect("unexpected null or other incorrect datatype for Trustfall type 'String!'")
                .clone();
            image::name_contains(contexts, substring, resolve_info)
        }
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Image'")
        }
    }
}

mod image {
    use super::*;
    use jiff::Timestamp;
    use trustfall::provider::{
        AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo, VertexIterator,
        resolve_neighbors_with,
    };

    use super::super::vertex::Vertex;

    pub(super) fn created_after<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        timestamp: &str,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        let ts = timestamp.parse::<Timestamp>().unwrap();

        resolve_neighbors_with(contexts, move |vertex| {
            let image = vertex
                .as_image()
                .expect("conversion failed, vertex was not a Image");

            if image.created_at > ts {
                Box::new(std::iter::once(vertex.clone()))
            } else {
                Box::new(std::iter::empty())
            }
        })
    }

    pub(super) fn created_before<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        timestamp: &str,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        let ts = timestamp.parse::<Timestamp>().unwrap();
        resolve_neighbors_with(contexts, move |vertex| {
            let image = vertex
                .as_image()
                .expect("conversion failed, vertex was not a Image");
            if image.created_at < ts {
                Box::new(std::iter::once(vertex.clone()))
            } else {
                Box::new(std::iter::empty())
            }
        })
    }

    pub(super) fn has_name<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        name: Arc<str>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let image = vertex
                .as_image()
                .expect("conversion failed, vertex was not a Image");
            if image.repository == name.as_ref()
                || format!("{}:{}", image.repository, image.tag) == name.as_ref()
            {
                Box::new(std::iter::once(vertex.clone()))
            } else {
                Box::new(std::iter::empty())
            }
        })
    }

    pub(super) fn name_contains<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        substr: Arc<str>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let image = vertex
                .as_image()
                .expect("conversion failed, vertex was not a Image");
            if image.repository.contains(substr.as_ref()) {
                Box::new(std::iter::once(vertex.clone()))
            } else {
                Box::new(std::iter::empty())
            }
        })
    }

    pub(super) fn name_matches<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        regex: Regex,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let image = vertex
                .as_image()
                .expect("conversion failed, vertex was not a Image");

            let name = format!("{}:{}", image.repository, image.tag);
            if regex.is_match(&name) {
                Box::new(std::iter::once(vertex.clone()))
            } else {
                Box::new(std::iter::empty())
            }
        })
    }

    pub(super) fn size_in_range<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        min: i64,
        max: i64,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let image = vertex
                .as_image()
                .expect("conversion failed, vertex was not a Image");
            let size = image.size as i64;
            if (min..max).contains(&size) {
                Box::new(std::iter::once(vertex.clone()))
            } else {
                Box::new(std::iter::empty())
            }
        })
    }
}
