use super::vertex::Vertex;
use std::sync::Arc;
use trustfall::{
    FieldValue,
    provider::{AsVertex, ContextIterator, ContextOutcomeIterator, DataContext, ResolveInfo},
};

pub(super) fn resolve_image_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    let func = match property_name {
        "created" => |v: DataContext<V>| match v.active_vertex() {
            Some(Vertex::Image(img)) => (
                v.clone(),
                FieldValue::String(Arc::from(img.created_at.to_string().as_str())),
            ),
            None => (v, FieldValue::Null),
            Some(v) => unreachable!("Invalid vertex: {:?}", v),
        },
        "repo" => |v: DataContext<V>| match v.active_vertex() {
            Some(Vertex::Image(img)) => (
                v.clone(),
                FieldValue::String(Arc::from(img.repository.as_str())),
            ),
            None => (v, FieldValue::Null),
            Some(v) => unreachable!("Invalid vertex: {:?}", v),
        },
        "tag" => |v: DataContext<V>| match v.active_vertex() {
            Some(Vertex::Image(img)) => {
                (v.clone(), FieldValue::String(Arc::from(img.tag.as_str())))
            }
            None => (v, FieldValue::Null),
            Some(v) => unreachable!("Invalid vertex: {:?}", v),
        },
        "size" => |v: DataContext<V>| match v.active_vertex() {
            Some(Vertex::Image(img)) => (v.clone(), FieldValue::Uint64(img.size as u64)),
            None => (v, FieldValue::Null),
            Some(v) => unreachable!("Invalid vertex: {:?}", v),
        },

        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Image'")
        }
    };
    Box::new(contexts.map(func))
}
