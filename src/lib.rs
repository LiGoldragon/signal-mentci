//! Ordinary Signal contract for Mentci.
//!
//! Mentci is the human-facing component: it projects interface state, presents
//! approval questions and records their verdicts, carries notification and
//! pane mutation, and shows the parked-request and intercept-policy views it
//! holds on Criome's behalf. Those Criome-owned shapes are imported from
//! `signal-criome`, never restated here.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against it. The
//! portable rkyv frame, its kinds, and the wire framing come from `signal`.

pub mod generated;
pub use generated::signal::*;

/// The portable rkyv frame and its three kinds, re-exported from `signal` so
/// a frame of this contract is the same Rust type as every other contract's
/// frame and a consumer need not name `signal` itself to speak this contract.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");
