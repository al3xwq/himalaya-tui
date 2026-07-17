//! First-time-setup wizard: provider discovery + credential prompts.
//!
//! [`discover`] is the entry point; [`pacc`], [`autoconfig`] and
//! [`srv`] implement the three discovery probes it chains together.

pub mod autoconfig;
pub mod discover;
pub mod pacc;
pub mod srv;
