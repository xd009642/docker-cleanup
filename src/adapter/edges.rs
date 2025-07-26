use trustfall::provider::{
    AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo,
    VertexIterator,
};

use super::vertex::Vertex;

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
            let name: &str = parameters
                .get("name")
                .expect("failed to find parameter 'name' for edge 'has_name' on type 'Image'")
                .as_str()
                .expect("unexpected null or other incorrect datatype for Trustfall type 'String!'");
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
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Image'")
        }
    }
}

mod image {
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
        name: &str,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_image()
                .expect("conversion failed, vertex was not a Image");
            todo!("get neighbors along edge 'has_name' for type 'Image'")
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
