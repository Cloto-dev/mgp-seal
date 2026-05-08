//! Integration tests for `mgp-seal`. Ported from the original ClotoCore
//! `mcp_seal.rs` test module — the cryptographic contract is the load-bearing
//! property of this crate, so the test surface is preserved verbatim.

use std::io::Write;

use mgp_seal::{
    check_seal, compute_seal, load_or_generate_seal_key, verify_seal, SealStatus, TrustLevel,
};
use tempfile::NamedTempFile;

/// Helper: create a temp file with given contents and return the path.
fn temp_file_with(content: &[u8]) -> NamedTempFile {
    let mut f = NamedTempFile::new().expect("create temp file");
    f.write_all(content).expect("write temp file");
    f.flush().expect("flush temp file");
    f
}

const TEST_KEY: &[u8] = b"test-seal-key-0123456789abcdef!";

// --------------------------------------------------------
// compute_seal
// --------------------------------------------------------

#[test]
fn compute_seal_returns_correct_format() {
    let file = temp_file_with(b"hello world");
    let seal = compute_seal(file.path(), TEST_KEY).unwrap();

    assert!(
        seal.starts_with("sha256:"),
        "seal should start with 'sha256:' but got: {seal}"
    );
    // "sha256:" = 7 chars, SHA-256 hex = 64 chars → total 71.
    assert_eq!(seal.len(), 71, "seal length mismatch: {seal}");
    // Hex portion should only contain hex characters.
    let hex_part = &seal[7..];
    assert!(
        hex_part.chars().all(|c| c.is_ascii_hexdigit()),
        "non-hex character in seal: {hex_part}"
    );
}

#[test]
fn compute_seal_deterministic() {
    let file = temp_file_with(b"deterministic content");
    let seal1 = compute_seal(file.path(), TEST_KEY).unwrap();
    let seal2 = compute_seal(file.path(), TEST_KEY).unwrap();
    assert_eq!(
        seal1, seal2,
        "same file + same key should produce same seal"
    );
}

#[test]
fn compute_seal_different_key_different_result() {
    let file = temp_file_with(b"same content");
    let seal_a = compute_seal(file.path(), b"key-alpha").unwrap();
    let seal_b = compute_seal(file.path(), b"key-bravo").unwrap();
    assert_ne!(
        seal_a, seal_b,
        "different keys should produce different seals"
    );
}

// --------------------------------------------------------
// verify_seal
// --------------------------------------------------------

#[test]
fn verify_seal_succeeds_with_correct_seal() {
    let file = temp_file_with(b"verify me");
    let seal = compute_seal(file.path(), TEST_KEY).unwrap();
    assert!(verify_seal(file.path(), &seal, TEST_KEY).unwrap());
}

#[test]
fn verify_seal_fails_with_wrong_seal() {
    let file = temp_file_with(b"verify me");
    let wrong = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
    assert!(!verify_seal(file.path(), wrong, TEST_KEY).unwrap());
}

#[test]
fn verify_seal_fails_with_wrong_key() {
    let file = temp_file_with(b"verify me");
    let seal = compute_seal(file.path(), TEST_KEY).unwrap();
    assert!(!verify_seal(file.path(), &seal, b"wrong-key").unwrap());
}

#[test]
fn verify_seal_rejects_invalid_format() {
    let file = temp_file_with(b"whatever");
    let result = verify_seal(file.path(), "md5:abcdef", TEST_KEY);
    assert!(result.is_err(), "non-sha256 prefix should error");
}

// --------------------------------------------------------
// check_seal — behavior matrix
// --------------------------------------------------------

/// Helper: create a sealed temp file and return (file, seal).
fn sealed_file() -> (NamedTempFile, String) {
    let file = temp_file_with(b"sealed binary content");
    let seal = compute_seal(file.path(), TEST_KEY).unwrap();
    (file, seal)
}

// --- Seal Present + Valid ---

#[test]
fn check_seal_core_valid_seal() {
    let (file, seal) = sealed_file();
    let status = check_seal(&TrustLevel::Core, Some(&seal), file.path(), TEST_KEY, false).unwrap();
    assert_eq!(status, SealStatus::Verified);
}

#[test]
fn check_seal_standard_valid_seal() {
    let (file, seal) = sealed_file();
    let status = check_seal(
        &TrustLevel::Standard,
        Some(&seal),
        file.path(),
        TEST_KEY,
        false,
    )
    .unwrap();
    assert_eq!(status, SealStatus::Verified);
}

#[test]
fn check_seal_experimental_valid_seal() {
    let (file, seal) = sealed_file();
    let status = check_seal(
        &TrustLevel::Experimental,
        Some(&seal),
        file.path(),
        TEST_KEY,
        false,
    )
    .unwrap();
    assert_eq!(status, SealStatus::Verified);
}

#[test]
fn check_seal_untrusted_valid_seal() {
    let (file, seal) = sealed_file();
    let status = check_seal(
        &TrustLevel::Untrusted,
        Some(&seal),
        file.path(),
        TEST_KEY,
        false,
    )
    .unwrap();
    assert_eq!(status, SealStatus::Verified);
}

// --- Seal Present + Invalid ---

#[test]
fn check_seal_core_invalid_seal() {
    let file = temp_file_with(b"binary");
    let bad = "sha256:badbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbad";
    let status = check_seal(&TrustLevel::Core, Some(bad), file.path(), TEST_KEY, false).unwrap();
    assert_eq!(status, SealStatus::Failed);
}

#[test]
fn check_seal_standard_invalid_seal() {
    let file = temp_file_with(b"binary");
    let bad = "sha256:badbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbad";
    let status = check_seal(
        &TrustLevel::Standard,
        Some(bad),
        file.path(),
        TEST_KEY,
        false,
    )
    .unwrap();
    assert_eq!(status, SealStatus::Failed);
}

#[test]
fn check_seal_experimental_invalid_seal() {
    let file = temp_file_with(b"binary");
    let bad = "sha256:badbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbad";
    let status = check_seal(
        &TrustLevel::Experimental,
        Some(bad),
        file.path(),
        TEST_KEY,
        false,
    )
    .unwrap();
    assert_eq!(status, SealStatus::Failed);
}

#[test]
fn check_seal_untrusted_invalid_seal() {
    let file = temp_file_with(b"binary");
    let bad = "sha256:badbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbad";
    let status = check_seal(
        &TrustLevel::Untrusted,
        Some(bad),
        file.path(),
        TEST_KEY,
        false,
    )
    .unwrap();
    assert_eq!(status, SealStatus::Failed);
}

// --- Seal Absent ---

#[test]
fn check_seal_core_absent_allows() {
    let file = temp_file_with(b"binary");
    let status = check_seal(&TrustLevel::Core, None, file.path(), TEST_KEY, false).unwrap();
    assert_eq!(status, SealStatus::Unsigned);
}

#[test]
fn check_seal_standard_absent_allows() {
    let file = temp_file_with(b"binary");
    let status = check_seal(&TrustLevel::Standard, None, file.path(), TEST_KEY, false).unwrap();
    assert_eq!(status, SealStatus::Unsigned);
}

#[test]
fn check_seal_experimental_absent_allows() {
    let file = temp_file_with(b"binary");
    let status = check_seal(
        &TrustLevel::Experimental,
        None,
        file.path(),
        TEST_KEY,
        false,
    )
    .unwrap();
    assert_eq!(status, SealStatus::Unsigned);
}

#[test]
fn check_seal_untrusted_absent_allows_v063() {
    // v0.6.3 §4.0 relaxation: an unsealed `Untrusted` server starts under
    // the same untrusted profile it would have anyway. Prior to v0.6.3
    // this returned `Failed` (Block); now it returns `Unsigned` (Allow).
    let file = temp_file_with(b"binary");
    let status = check_seal(&TrustLevel::Untrusted, None, file.path(), TEST_KEY, false).unwrap();
    assert_eq!(status, SealStatus::Unsigned);
}

// --- Untrusted + allow_unsigned ---

#[test]
fn check_seal_untrusted_absent_allow_unsigned_skips() {
    let file = temp_file_with(b"binary");
    let status = check_seal(&TrustLevel::Untrusted, None, file.path(), TEST_KEY, true).unwrap();
    assert_eq!(status, SealStatus::Skipped);
}

#[test]
fn check_seal_untrusted_invalid_seal_allow_unsigned_still_fails() {
    let file = temp_file_with(b"binary");
    let bad = "sha256:badbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbad";
    let status = check_seal(
        &TrustLevel::Untrusted,
        Some(bad),
        file.path(),
        TEST_KEY,
        true,
    )
    .unwrap();
    assert_eq!(
        status,
        SealStatus::Failed,
        "allow_unsigned should not bypass invalid seal verification"
    );
}

// --------------------------------------------------------
// load_or_generate_seal_key
// --------------------------------------------------------

#[test]
fn load_or_generate_creates_key_file() {
    // Guard against env var pollution from parallel test (load_or_generate_reads_env_var)
    let env_guard = std::env::var("CLOTO_SEAL_KEY").ok();
    std::env::remove_var("CLOTO_SEAL_KEY");

    let dir = tempfile::tempdir().unwrap();
    let key = load_or_generate_seal_key(dir.path()).unwrap();
    assert_eq!(key.len(), 32);

    let key_path = dir.path().join("seal.key");
    assert!(key_path.exists(), "seal.key should be created");

    // Loading again should return the same key (from file, not env).
    let key2 = load_or_generate_seal_key(dir.path()).unwrap();
    assert_eq!(key, key2, "subsequent load should return same key");

    // Restore env var if it was set before
    if let Some(val) = env_guard {
        std::env::set_var("CLOTO_SEAL_KEY", val);
    }
}

#[test]
fn load_or_generate_reads_env_var() {
    let dir = tempfile::tempdir().unwrap();
    let hex_key = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

    // Temporarily set the env var. This is not thread-safe but acceptable in unit tests.
    std::env::set_var("CLOTO_SEAL_KEY", hex_key);
    let key = load_or_generate_seal_key(dir.path()).unwrap();
    std::env::remove_var("CLOTO_SEAL_KEY");

    assert_eq!(key, hex::decode(hex_key).unwrap());
    // Should NOT have created a file since env var was used.
    assert!(
        !dir.path().join("seal.key").exists(),
        "seal.key should not be created when env var is set"
    );
}
