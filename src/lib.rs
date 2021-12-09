pub mod config;
pub mod core;
pub mod error;
pub mod telemetry;

mod sealed {
    pub trait Sealed<A = ()> {}
}
