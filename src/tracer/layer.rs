use std::collections::BTreeMap;

use serde_json::json;
use tracing::span::{Attributes, Record};
use tracing::{Event, Id};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{prelude::*, Layer};

use crate::tracer::store::TracingFieldStore;
use crate::tracer::visitor::TracingFieldVisitor;

pub(crate) struct TracingFormatLayer;

impl<S> Layer<S> for TracingFormatLayer
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let new_span = ctx.span(id);
        if let Some(span) = new_span {
            let mut fields = BTreeMap::new();
            let mut visitor = TracingFieldVisitor { inner: &mut fields };
            attrs.record(&mut visitor);

            let store = TracingFieldStore { inner: fields };

            let mut extensions = span.extensions_mut();
            extensions.insert(store);
        }
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let recorded_span = ctx.span(span);
        if let Some(span) = recorded_span {
            let mut extensions = span.extensions_mut();

            if let Some(store) = extensions.get_mut::<TracingFieldStore>() {
                let mut visitor = TracingFieldVisitor {
                    inner: &mut store.inner,
                };
                values.record(&mut visitor);
            }
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let event_scope = ctx.event_scope(event);

        if let Some(scope) = event_scope {
            let mut registered = vec![];

            for span in scope.from_root() {
                let extensions = span.extensions();

                if let Some(store) = extensions.get::<TracingFieldStore>() {
                    let field_map = &store.inner;
                    registered.push(json!({
                        "target": span.metadata().target(),
                        "name": span.metadata().name(),
                        "level": span.metadata().level().to_string(),
                        "fields": field_map,
                    }));
                }
            }

            let mut fields = BTreeMap::new();
            let mut visitor = TracingFieldVisitor { inner: &mut fields };
            event.record(&mut visitor);

            let output = json!({
                "target": event.metadata().target(),
                "name": event.metadata().name(),
                "level": event.metadata().level().to_string(),
                "fields": fields,
                "registered": registered,
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
    }
}

// pub(crate) type VisitorMap<'a, 'b> = &'a mut BTreeMap<&'b str, serde_json::Value>;

/// Initializes the logger and sets options for `tracing-subscriber` output.
pub fn init_trace_logger() {
    // Set up `tracing-subscriber` will deal with tracing data
    tracing_subscriber::registry()
        .with(TracingFormatLayer)
        .init();
}
