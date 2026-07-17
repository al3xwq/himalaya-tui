//! SMTP backend: a thin wrapper around
//! [`io_smtp::client::SmtpClientStd`] plus the send adapter for the
//! shared cross-protocol client, inlined from the retired io-email
//! crate.

pub mod backend;
pub mod client;
