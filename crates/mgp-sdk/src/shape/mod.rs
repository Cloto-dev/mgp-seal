//! Shape conversion: `cloto-connector.json` → `registry.json` entry.
//!
//! ClotoHub.dev's catalog endpoint serves `registry.json` in the form
//! ClotoCore already consumes today (`Registry { schema_version,
//! updated_at, servers: [RegistryEntry, ...] }`). The conversion
//! happens here so both ClotoHub.dev (synthesizer) and ClotoCore
//! (direct fallback resolver) emit byte-compatible payloads.

mod registry;

pub use registry::{manifest_to_registry_entry, InstallShape, RegistryEntry};
