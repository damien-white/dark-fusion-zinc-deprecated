//! Zinc is a key-value store meant to be lightweight, easy-to-use and flexible
//! in its application. It aims to provide a performant solution that runs
//! efficiently on IoT devices and servers that have access to only a single CPU
//! core, limited power, or other resource constraints.
//!
//! The database engine is built on top of the tokio runtime and leverages
//! concurrency primitives from [`parking_lot`] and [`dashmap`]. The engine is
//! designed to support single-core systems and is single-threaded by default.
//!
//! Fusion Core uses various data structures and
//! concurrency primitives provided by both the standard library and third-party
//! crates such as [`dashmap`], [`futures`] and [`parking_lot`].
//!
//! [`dashmap`]: https://docs.rs/dashmap/latest/dashmap/
//! [`futures`]: https://docs.rs/futures/latest/futures
//! [`parking_lot`]: https://docs.rs/parking_lot/latest/parking_lot/
//!
//! __WARNING: This project is very unstable. Please do not attempt to use it in
//! production environments.__
pub mod client;
pub mod logger;
pub mod server;
