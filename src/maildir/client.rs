//! Himalaya TUI wrapper around [`io_maildir::client::MaildirClient`].
//!
//! Built from the per-account [`MaildirConfig`] block and handed to the
//! shared cross-protocol client. The configured filesystem root is kept
//! on the wrapper so commands can join per-mailbox sub-paths.

use std::{
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use anyhow::Result;
use io_maildir::{client::MaildirClient as Inner, maildir::Maildir};

use crate::config::MaildirConfig;

/// Live Maildir client wrapping io_maildir with the configured root.
pub struct MaildirClient {
    inner: Inner,
    /// Filesystem root of the configured maildir. Kept on the wrapper
    /// so commands can join sub-paths (per-mailbox) without needing the
    /// original [`MaildirConfig`].
    pub root: PathBuf,
}

impl MaildirClient {
    /// Builds a [`MaildirClient`] rooted at the configured maildir path.
    pub fn new(config: MaildirConfig) -> Self {
        let root = config.root.clone();
        let inner = Inner::new(root.to_string_lossy().into_owned());
        Self { inner, root }
    }

    /// Resolves a maildir CLI argument: tries `path` as-is first, then
    /// falls back to `self.root.join(path)`. Both attempts go through
    /// [`io_maildir::client::MaildirClient::load_maildir`] so the `cur`
    /// / `new` / `tmp` markers are validated.
    pub fn resolve_maildir(&self, path: &Path) -> Result<Maildir> {
        if let Ok(maildir) = self.load_maildir(path.to_string_lossy().into_owned()) {
            return Ok(maildir);
        }
        Ok(self.load_maildir(self.root.join(path).to_string_lossy().into_owned())?)
    }
}

impl Deref for MaildirClient {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for MaildirClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
