use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::tracer::visitor::TracingFieldVisitor;
use serde_json::json;
use tracing::span::Attributes;
use tracing::{Event, Id};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{prelude::*, Layer};

// TODO: Split this into a separate crate

pub(crate) type TraceFieldMap<'a> = BTreeMap<&'a str, serde_json::Value>;

#[derive(Clone, Debug)]
pub struct TracingFieldStore<'a> {
    inner: TraceFieldMap<'a>,
}

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

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let event_scope = ctx.event_scope(event);

        if let Some(scope) = event_scope {
            let mut registered = vec![];

            for span in scope.from_root() {
                let extensions = span.extensions();
                if let Some(store) = extensions.get::<TracingFieldStore>() {
                    let field_map = &store.inner;
                    registered.push(json!({
                        "target": event.metadata().target(),
                        "name": event.metadata().name(),
                        "level": format!("{:?}", event.metadata().level()),
                        "fields": field_map,
                    }));
                }
            }
        }

        let mut fields = BTreeMap::new();
        let mut visitor = TracingFieldVisitor { inner: &mut fields };
        event.record(&mut visitor);

        // let metadata = event.metadata();
        let output = serde_json::json!({
            "target": event.metadata().target(),
            "name": event.metadata().name(),
            "level": format!("{:?}", event.metadata().level()),
            "fields": fields,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
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

// Initialize console and file tracing loggers at application startup.
//
// Provides both per-layer and global filtering by leveraging `Targets`.
// pub fn initialize_logger() -> std::io::Result<()> {
//     let console_logger = tracing_subscriber::fmt::layer()
//         .pretty()
//         .with_filter(Targets::new().with_default(Level::TRACE));
//
//     let log_file = File::create("zinc.log")?;
//     let file_logger = tracing_subscriber::fmt::layer()
//         .json()
//         .with_writer(Arc::new(log_file));
//
//     Registry::default()
//         .with(console_logger)
//         .with(file_logger)
//         .with(
//             Targets::default()
//                 .with_target("zinc", Level::TRACE)
//                 .with_target("tokio", Level::DEBUG),
//         )
//         .init();
//
//     Ok(())
// }
