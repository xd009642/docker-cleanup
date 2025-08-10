use super::vertex::Vertex;
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
            Some(Vertex::Image(img)) => (v.clone(), img.created_at.to_string().into()),
            None => (v, FieldValue::Null),
        },
        "repo" => |v: DataContext<V>| match v.active_vertex() {
            Some(Vertex::Image(img)) => {
                let value = if img.repository.is_empty() {
                    FieldValue::Null
                } else {
                    img.repository.as_str().into()
                };
                (v.clone(), value)
            }
            None => (v, FieldValue::Null),
        },
        "tag" => |v: DataContext<V>| match v.active_vertex() {
            Some(Vertex::Image(img)) => {
                let value = if img.tag.as_str().is_empty() {
                    FieldValue::Null
                } else {
                    img.tag.as_str().into()
                };
                (v.clone(), value)
            }
            None => (v, FieldValue::Null),
        },
        "name" => |v: DataContext<V>| match v.active_vertex() {
            Some(Vertex::Image(img)) => {
                let value = if img.repository.is_empty() {
                    FieldValue::Null
                } else {
                    if img.tag.is_empty() {
                        img.repository.as_str().into()
                    } else {
                        let name = format!("{}:{}", img.repository, img.tag);
                        name.into()
                    }
                };
                (v.clone(), value)
            }
            None => (v, FieldValue::Null),
        },

        "size" => |v: DataContext<V>| match v.active_vertex() {
            Some(Vertex::Image(img)) => (v.clone(), FieldValue::Uint64(img.size as u64)),
            None => (v, FieldValue::Null),
        },
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Image'")
        }
    };
    Box::new(contexts.map(func))
}
