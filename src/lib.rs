//! Authority-verified ordinary Mentci Interface.
//!
//! Ethos owns visible contract vocabulary; Rust exposes only encoded coordinates.
pub mod bootstrap_manifest;
pub mod schema;
pub const INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");
pub const INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");
pub use schema::lib::*;
