//! `docker` source adapter.

use serde::{Deserialize, Serialize};

/// Docker image reference. Consumer materializes via `docker pull`
/// (or any OCI-compatible client).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DockerSpec {
    /// Image name. Includes registry host and namespace if non-default
    /// (e.g., `ghcr.io/cloto-dev/foo`); short form (e.g., `nginx`)
    /// resolves against Docker Hub.
    pub image: String,
    /// Tag or digest. Empty / `None` falls back to `latest` (consumers
    /// SHOULD reject `latest` for production registration).
    #[serde(default)]
    pub tag: Option<String>,
}

impl DockerSpec {
    /// Render the canonical reference (`image:tag` or `image@digest`).
    #[must_use]
    pub fn canonical_reference(&self) -> String {
        match &self.tag {
            Some(t) if !t.is_empty() => format!("{}:{}", self.image, t),
            _ => format!("{}:latest", self.image),
        }
    }

    /// `true` when the tag would resolve to `latest` (= unpinned).
    #[must_use]
    pub fn is_unpinned(&self) -> bool {
        match &self.tag {
            Some(t) => t.is_empty() || t == "latest",
            None => true,
        }
    }

    /// Validate the image string is non-empty and has no whitespace.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the image is empty or contains whitespace.
    pub fn check(&self) -> Result<(), &'static str> {
        if self.image.is_empty() {
            return Err("docker image is empty");
        }
        if self.image.chars().any(char::is_whitespace) {
            return Err("docker image contains whitespace");
        }
        Ok(())
    }
}
