//! Rust types for the `cloto-connector.json` v1 manifest schema.
//!
//! These mirror the schema declared in `project_clotohub_design.md` §
//! cloto-connector.json. Unknown fields are ignored on deserialize to
//! preserve forward-compat (v1 → v2 additive evolution); known fields
//! are preserved on serialize.

use crate::adapters::SourceSpec;
use serde::{Deserialize, Serialize};

/// Top-level cloto-connector.json document (v1).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectorManifest {
    /// Manifest schema version. Must equal `1` for v1.
    pub spec_version: u32,
    /// Connector kind. v1 only accepts `"mgp_server"`.
    pub connector_type: String,
    /// Stable connector identifier (kebab-case).
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Short description.
    pub description: String,
    /// Connector version (SemVer).
    pub version: String,
    /// Marketplace category.
    pub category: String,
    /// MGP §2.3 trust tier: `core | standard | experimental | untrusted`.
    pub trust_level: String,
    /// MGP §8 L0 Magic Seal in `sha256:HEX` form. Required at registration.
    pub magic_seal: String,
    /// Install/runtime declaration.
    pub install: InstallSpec,
    /// Optional UI metadata.
    #[serde(default)]
    pub icon: Option<String>,
    /// Tag set for marketplace filtering.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Hosts that can run this connector
    /// (`clotocore | claude-code | claude-desktop | ...`).
    #[serde(default)]
    pub host_compatibility: Vec<String>,
    /// Required environment variables.
    #[serde(default)]
    pub env_vars: Vec<EnvVarDef>,
    /// Optional environment variables.
    #[serde(default)]
    pub optional_env_vars: Vec<EnvVarDef>,
    /// Auto-restart policy for the host.
    #[serde(default)]
    pub auto_restart: bool,
    /// Optional CHANGELOG content.
    #[serde(default)]
    pub changelog: Option<String>,
}

/// Install / runtime declaration block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstallSpec {
    /// Source descriptor — see [`SourceSpec`].
    pub source: SourceSpec,
    /// Package manager used to materialize the connector. v1: `"uv"`.
    pub package_manager: String,
    /// Runtime: `python | rust | node`.
    pub runtime: String,
    /// Optional list of extra dependencies the host should resolve.
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// Subdirectory inside the source tree where the connector lives.
    #[serde(default)]
    pub directory: String,
    /// Binary name produced by the build (relevant for `runtime = rust`).
    #[serde(default)]
    pub bin_name: Option<String>,
}

/// Environment variable contract entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnvVarDef {
    /// Variable name.
    pub name: String,
    /// Human description.
    #[serde(default)]
    pub description: Option<String>,
}
