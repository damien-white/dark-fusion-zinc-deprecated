//! Telemetry module capable of storing, formatting and displaying trace logs.

// TODO: Move module to separate crate so it can be used in other projects

pub use layer::init_trace_logger;

pub mod layer;
pub mod store;
pub mod visitor;
