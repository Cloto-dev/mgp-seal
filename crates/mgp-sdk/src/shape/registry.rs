//! Conversion of [`ConnectorManifest`] into the `registry.json` entry
//! shape that ClotoCore already consumes today.
//!
//! This intentionally **mirrors** ClotoCore's `RegistryEntry` field
//! layout. Keep the two in lock-step until ClotoCore migrates to
//! importing this type directly (post Phase 5c cutover).

use crate::types::{ConnectorManifest, EnvVarDef};
use serde::{Deserialize, Serialize};

/// Single entry in the catalog `registry.json` file.
///
/// Field order, names, and `#[serde(default)]` annotations are
/// load-bearing — they must match `cloto_core::handlers::marketplace::
/// RegistryEntry` exactly so the same JSON parses both sides.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegistryEntry {
    /// Stable connector id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Short description.
    pub description: String,
    /// Marketplace category.
    pub category: String,
    /// SemVer.
    pub version: String,
    /// Source-tree subdirectory.
    #[serde(default)]
    pub directory: String,
    /// Extra dependency hints.
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// Required env vars.
    #[serde(default)]
    pub env_vars: Vec<EnvVarDef>,
    /// Optional env vars.
    #[serde(default)]
    pub optional_env_vars: Vec<EnvVarDef>,
    /// Marketplace tags.
    #[serde(default)]
    pub tags: Vec<String>,
    /// MGP §2.3 trust tier.
    #[serde(default = "default_trust_level")]
    pub trust_level: String,
    /// Host auto-restart policy.
    #[serde(default)]
    pub auto_restart: bool,
    /// Optional icon.
    #[serde(default)]
    pub icon: Option<String>,
    /// Runtime kind.
    #[serde(default = "default_runtime")]
    pub runtime: String,
    /// Optional binary name (for `runtime = rust`).
    #[serde(default)]
    pub bin_name: Option<String>,
    /// Optional changelog.
    #[serde(default)]
    pub changelog: Option<String>,
    /// MGP §8 L0 Magic Seal in `sha256:HEX` form.
    #[serde(default)]
    pub seal: Option<String>,
}

fn default_trust_level() -> String {
    "standard".to_string()
}

fn default_runtime() -> String {
    "python".to_string()
}

/// Convert a [`ConnectorManifest`] into a [`RegistryEntry`].
///
/// The Magic Seal is preserved as-is; it is the responsibility of the
/// emitter (ClotoHub.dev sync worker) to persist a `seals.revoked_at
/// IS NULL` row before serializing the entry. ClotoCore's runtime
/// re-checks the seal independently, so a stale seal here is caught
/// by §10 inv 3 force-untrusted at connect time.
#[must_use]
pub fn manifest_to_registry_entry(m: &ConnectorManifest) -> RegistryEntry {
    RegistryEntry {
        id: m.id.clone(),
        name: m.name.clone(),
        description: m.description.clone(),
        category: m.category.clone(),
        version: m.version.clone(),
        directory: m.install.directory.clone(),
        dependencies: m.install.dependencies.clone(),
        env_vars: m.env_vars.clone(),
        optional_env_vars: m.optional_env_vars.clone(),
        tags: m.tags.clone(),
        trust_level: m.trust_level.clone(),
        auto_restart: m.auto_restart,
        icon: m.icon.clone(),
        runtime: m.install.runtime.clone(),
        bin_name: m.install.bin_name.clone(),
        changelog: m.changelog.clone(),
        seal: Some(m.magic_seal.clone()),
    }
}
