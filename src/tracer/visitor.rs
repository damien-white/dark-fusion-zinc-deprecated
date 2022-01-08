use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use tracing::field::{Field, Visit};

pub(crate) struct TracingFieldVisitor<'a, 'b> {
    pub(crate) inner: &'a mut BTreeMap<&'b str, serde_json::Value>,
}

impl<'a, 'b> Visit for TracingFieldVisitor<'a, 'b> {
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.inner.insert(field.name(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.inner.insert(field.name(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.inner.insert(field.name(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.inner.insert(field.name(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.inner.insert(field.name(), serde_json::json!(value));
    }

    fn record_error(&mut self, field: &Field, value: &(dyn Error + 'static)) {
        self.inner
            .insert(field.name(), serde_json::json!(value.to_string()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.inner
            .insert(field.name(), serde_json::json!(format!("{:?}", value)));
    }
}
