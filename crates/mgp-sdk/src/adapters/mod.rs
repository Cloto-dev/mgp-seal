//! Source-spec adapters (`git | raw_url | pypi | docker`) — pure config
//! logic, no network IO. Each adapter parses and validates its
//! sub-spec; consumers (ClotoHub.dev sync worker, ClotoCore install
//! path) execute the actual fetch.

use serde::{Deserialize, Serialize};

mod docker;
mod git;
mod pypi;
mod raw_url;

pub use docker::DockerSpec;
pub use git::GitSpec;
pub use pypi::PypiSpec;
pub use raw_url::RawUrlSpec;

/// Source descriptor — the `install.source` field of `ConnectorManifest`.
///
/// `type` (kebab-case) selects the variant; the rest of the fields are
/// flattened into the inner spec struct.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SourceSpec {
    /// Git repository, optionally pinned to a ref.
    Git(GitSpec),
    /// Single HTTP(S) artifact.
    RawUrl(RawUrlSpec),
    /// PyPI package (resolved by `uv pip install`).
    Pypi(PypiSpec),
    /// Docker image.
    Docker(DockerSpec),
}

impl SourceSpec {
    /// Discriminant string, matching the `type` tag in JSON.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Git(_) => "git",
            Self::RawUrl(_) => "raw_url",
            Self::Pypi(_) => "pypi",
            Self::Docker(_) => "docker",
        }
    }
}
