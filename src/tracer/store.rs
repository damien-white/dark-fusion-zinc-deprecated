use std::collections::BTreeMap;

/// Type alias for a `Map` that uses `&str` keys to index arbitrary JSON values
pub(crate) type TraceFieldMap<'a> = BTreeMap<&'a str, serde_json::Value>;

#[derive(Clone, Debug)]
pub struct TracingFieldStore<'a> {
    pub(crate) inner: TraceFieldMap<'a>,
}
