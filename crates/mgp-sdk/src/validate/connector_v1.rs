//! v1 manifest validator.

use super::ValidationError;
use crate::adapters::SourceSpec;
use crate::types::ConnectorManifest;
use crate::{CONNECTOR_TYPE_MGP_SERVER, PACKAGE_MANAGER_UV, SPEC_VERSION};

const TRUST_LEVELS: &[&str] = &["core", "standard", "experimental", "untrusted"];
const RUNTIMES: &[&str] = &["python", "rust", "node"];

/// Validate a `cloto-connector.json` v1 manifest. Pure logic — no IO.
///
/// # Errors
///
/// Returns the first [`ValidationError`] encountered. Validation
/// short-circuits; full-detail diagnostics belong to ClotoHub.dev's
/// admin tooling, which can call the underlying check helpers
/// directly.
pub fn validate_v1(manifest: &ConnectorManifest) -> Result<(), ValidationError> {
    if manifest.spec_version != SPEC_VERSION {
        return Err(ValidationError::UnsupportedSpecVersion(
            manifest.spec_version,
        ));
    }
    if manifest.connector_type != CONNECTOR_TYPE_MGP_SERVER {
        return Err(ValidationError::UnsupportedConnectorType(
            manifest.connector_type.clone(),
        ));
    }
    if !is_kebab_case_id(&manifest.id) {
        return Err(ValidationError::InvalidId);
    }
    if !TRUST_LEVELS.contains(&manifest.trust_level.as_str()) {
        return Err(ValidationError::UnsupportedTrustLevel(
            manifest.trust_level.clone(),
        ));
    }
    if !is_well_formed_seal(&manifest.magic_seal) {
        return Err(ValidationError::MalformedMagicSeal);
    }
    if manifest.install.package_manager != PACKAGE_MANAGER_UV {
        return Err(ValidationError::UnsupportedPackageManager(
            manifest.install.package_manager.clone(),
        ));
    }
    if !RUNTIMES.contains(&manifest.install.runtime.as_str()) {
        return Err(ValidationError::UnsupportedRuntime(
            manifest.install.runtime.clone(),
        ));
    }
    validate_source(&manifest.install.source)?;
    Ok(())
}

fn validate_source(source: &SourceSpec) -> Result<(), ValidationError> {
    let kind = source.kind();
    let reason = match source {
        SourceSpec::Git(s) => s.check_url().err(),
        SourceSpec::RawUrl(s) => s.check().err(),
        SourceSpec::Pypi(s) => s.check().err(),
        SourceSpec::Docker(s) => s.check().err(),
    };
    if let Some(reason) = reason {
        return Err(ValidationError::InvalidSource { kind, reason });
    }
    Ok(())
}

fn is_kebab_case_id(id: &str) -> bool {
    if id.is_empty() {
        return false;
    }
    id.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !id.starts_with('-')
        && !id.ends_with('-')
}

fn is_well_formed_seal(seal: &str) -> bool {
    let Some(hex) = seal.strip_prefix("sha256:") else {
        return false;
    };
    hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit())
}
