//! Himalaya TUI wrapper around [`io_m2dir::client::M2dirClient`].
//!
//! Built from the per-account [`M2dirConfig`] block and handed to the
//! shared cross-protocol client. m2dir resolves each mailbox against
//! the configured store root internally, so the wrapper keeps nothing
//! beyond the inner client.

use std::ops::{Deref, DerefMut};

use io_m2dir::client::M2dirClient as Inner;

use crate::config::M2dirConfig;

/// Live m2dir client wrapping io_m2dir with the configured store root.
pub struct M2dirClient {
    inner: Inner,
}

impl M2dirClient {
    /// Builds an [`M2dirClient`] rooted at the configured m2store path.
    pub fn new(config: M2dirConfig) -> Self {
        let inner = Inner::new(config.root.to_string_lossy().into_owned());
        Self { inner }
    }
}

impl Deref for M2dirClient {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for M2dirClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
