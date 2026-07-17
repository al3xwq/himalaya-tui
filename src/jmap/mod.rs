//! JMAP backend: a thin wrapper around io_jmap's high-level client
//! plus the adapter that lowers its responses into the shared
//! cross-protocol email domain types.
//!
//! [`client`] holds the [`client::JmapClient`] session wrapper;
//! [`backend`] holds the `impl JmapClient` adapter methods the shared
//! interface calls.

pub mod backend;
pub mod client;
