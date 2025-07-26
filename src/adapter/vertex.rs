use std::sync::Arc;

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Image(Arc<crate::Image>),
}
