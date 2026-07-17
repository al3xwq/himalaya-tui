//! Shared cross-protocol email domain types, inlined from the retired
//! io-email crate.
//!
//! io-email is no longer a dependency: himalaya-tui owns its shared
//! types and a per-backend dispatching client instead, mirroring the
//! himalaya CLI. These modules hold the least-common-denominator shapes
//! the interface renders; the per-backend adapters that produce them
//! live in each protocol module's `backend` submodule.

pub mod address;
pub mod envelope;
pub mod flag;
pub mod mailbox;
