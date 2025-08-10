use super::vertex::Vertex;
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
}
