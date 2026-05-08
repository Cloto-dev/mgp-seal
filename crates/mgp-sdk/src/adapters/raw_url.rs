//! `raw_url` source adapter.

use serde::{Deserialize, Serialize};

/// Raw HTTP(S) artifact (typically a tarball or zip). The optional
/// `sha256` lets the consumer verify integrity before extraction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawUrlSpec {
    /// Direct download URL (http or https).
    pub url: String,
    /// Hex-encoded SHA-256 of the artifact. Optional but strongly
    /// recommended for v1; required for assets the consumer cannot
    /// re-derive (binary releases, frozen tarballs).
    #[serde(default)]
    pub sha256: Option<String>,
}

impl RawUrlSpec {
    /// Validate the URL parses as `http(s)` and the sha256 (if any) is
    /// 64 hex chars.
    ///
    /// # Errors
    ///
    /// Returns `Err` with a brief reason when the URL scheme is not
    /// `http` / `https` or the sha256 is malformed.
    pub fn check(&self) -> Result<(), &'static str> {
        let parsed = url::Url::parse(&self.url).map_err(|_| "raw_url url is not parseable")?;
        if !matches!(parsed.scheme(), "http" | "https") {
            return Err("raw_url scheme must be http or https");
        }
        if let Some(hash) = &self.sha256 {
            if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err("raw_url sha256 must be 64 hex characters");
            }
        }
        Ok(())
    }
}
