//! m2dir backend: a thin wrapper around io-m2dir's high-level client
//! plus the adapter that maps its filesystem entries onto the shared
//! [`crate::email`] domain types.

pub mod backend;
pub mod client;
