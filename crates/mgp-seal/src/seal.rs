//! Magic Seal cryptographic primitives — HMAC-SHA256 over a server binary.

use std::path::Path;

use anyhow::{bail, Context};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::trust::TrustLevel;

type HmacSha256 = Hmac<Sha256>;

/// Seal prefix used in the `"sha256:{hex}"` format.
const SEAL_PREFIX: &str = "sha256:";

// ============================================================
// Types
// ============================================================

/// Result of a seal verification check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SealStatus {
    /// Seal verified successfully.
    Verified,
    /// Seal verification failed (tampered or wrong key).
    Failed,
    /// No seal provided but allowed by trust level.
    Unsigned,
    /// Seal check skipped (development mode).
    Skipped,
}

// ============================================================
// Core Functions
// ============================================================

/// Compute HMAC-SHA256 of a file's contents.
/// Returns `"sha256:{hex_digest}"` format.
pub fn compute_seal(file_path: &Path, key: &[u8]) -> anyhow::Result<String> {
    let data = std::fs::read(file_path)
        .with_context(|| format!("Failed to read file for sealing: {}", file_path.display()))?;

    let mut mac =
        HmacSha256::new_from_slice(key).context("Invalid HMAC key length (should not happen)")?;
    mac.update(&data);
    let result = mac.finalize();
    let digest = hex::encode(result.into_bytes());

    Ok(format!("{SEAL_PREFIX}{digest}"))
}

/// Verify a file's HMAC-SHA256 against a stored seal value.
///
/// Returns `true` if the computed seal matches `expected_seal`, `false` otherwise.
/// Returns an error only on I/O or format problems — a mismatch is not an error.
pub fn verify_seal(file_path: &Path, expected_seal: &str, key: &[u8]) -> anyhow::Result<bool> {
    if !expected_seal.starts_with(SEAL_PREFIX) {
        bail!(
            "Invalid seal format: expected 'sha256:...' but got '{}'",
            expected_seal
        );
    }

    let computed = compute_seal(file_path, key)?;
    // Constant-time comparison of the hex strings to avoid timing side-channels.
    Ok(subtle::ConstantTimeEq::ct_eq(computed.as_bytes(), expected_seal.as_bytes()).into())
}

/// Check seal status based on trust level and configuration.
///
/// This function only computes the cryptographic outcome. Translating the
/// outcome into a startup decision (block / start under untrusted profile /
/// start under declared profile) is the caller's job — see the v0.6.3
/// behavior table below.
///
/// Behavior matrix (MGP_ISOLATION_DESIGN.md §4.0, v0.6.3+):
///
/// | Trust Level   | Seal Present | Seal Absent       | Seal Invalid |
/// |---------------|--------------|-------------------|--------------|
/// | Core          | Verify       | Force `untrusted` | Block        |
/// | Standard      | Verify       | Force `untrusted` | Block        |
/// | Experimental  | Verify       | Force `untrusted` | Block        |
/// | Untrusted     | Verify       | Allow*            | Block        |
///
/// "Force `untrusted`" means: this function returns [`SealStatus::Unsigned`];
/// the caller MUST override the effective trust_level to `Untrusted` (so the
/// isolation profile is pinned to the untrusted baseline) and emit a
/// `TRUST_LEVEL_DOWNGRADED_NO_SEAL` audit event (MGP_SECURITY.md §6.4).
///
/// * `Untrusted` + seal absent: returns [`SealStatus::Skipped`] when
///   `allow_unsigned=true` (dev mode), [`SealStatus::Unsigned`] otherwise.
///   Either way the effective trust_level is already `Untrusted`, so no
///   override is needed; the caller still allows startup.
///
/// `Seal invalid` (tampering) returns [`SealStatus::Failed`] across all tiers
/// — the caller MUST block startup. v0.6.3 only relaxed the `Seal absent` row.
pub fn check_seal(
    trust_level: &TrustLevel,
    seal_value: Option<&str>,
    entry_point: &Path,
    seal_key: &[u8],
    allow_unsigned: bool,
) -> anyhow::Result<SealStatus> {
    match seal_value {
        Some(seal) => {
            // Seal present — verify for all trust levels.
            let valid = verify_seal(entry_point, seal, seal_key)?;
            if valid {
                Ok(SealStatus::Verified)
            } else {
                // Invalid seal — block regardless of trust level.
                // For Core/Standard we also warn (caller should log at WARN level).
                Ok(SealStatus::Failed)
            }
        }
        None => {
            // Seal absent — behavior depends on trust level.
            //
            // v0.6.3: every tier returns `Unsigned` here; the caller forces the
            // effective trust_level to `Untrusted` regardless of declared tier.
            // Prior to v0.6.3 we returned `Failed` for `Untrusted` in production
            // (which blocked startup); v0.6.3 §4.0 relaxed that so an unsealed
            // `Untrusted` server starts under the same untrusted profile it
            // would have anyway. `Skipped` is reserved for dev-mode bypass.
            if matches!(trust_level, TrustLevel::Untrusted) && allow_unsigned {
                // Dev-mode bypass for already-untrusted: caller may keep the
                // declared profile. Emitted only for parity with prior versions.
                Ok(SealStatus::Skipped)
            } else {
                Ok(SealStatus::Unsigned)
            }
        }
    }
}
