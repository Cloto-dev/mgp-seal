//! `git` source adapter.

use serde::{Deserialize, Serialize};

/// Git source: clone `url` and check out `reference`. Default ref is
/// the upstream default branch (resolved at fetch time).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitSpec {
    /// Clone URL (https or ssh — consumers may restrict).
    pub url: String,
    /// Branch / tag / commit SHA. Empty string means upstream default.
    #[serde(default)]
    pub reference: String,
    /// Optional subdirectory inside the cloned tree where the manifest
    /// applies. Mirrors `InstallSpec.directory` for consumers that
    /// need the source-side hint.
    #[serde(default)]
    pub subdir: Option<String>,
}

impl GitSpec {
    /// Returns `true` when `reference` is empty (= upstream default).
    #[must_use]
    pub fn uses_default_ref(&self) -> bool {
        self.reference.is_empty()
    }

    /// Validate the URL is parseable as a URL or an `scp`-style git ref.
    /// Pure validation only; does not contact the network.
    ///
    /// # Errors
    ///
    /// Returns `Err` with a brief reason when the URL is empty or
    /// neither a parseable URL nor an `scp`-style git ref.
    pub fn check_url(&self) -> Result<(), &'static str> {
        if self.url.is_empty() {
            return Err("git url is empty");
        }
        // Allow scp-style ssh remotes (`git@host:owner/repo.git`).
        if self.url.starts_with("git@") {
            return Ok(());
        }
        url::Url::parse(&self.url)
            .map(|_| ())
            .map_err(|_| "git url is not parseable")
    }
}
