//! Integration tests for the Ed25519 public API.
//!
//! Unit tests in `src/ed25519.rs` exercise the internals; these tests pin the
//! end-to-end contract a downstream consumer (ClotoCore, ClotoHub) sees:
//!
//! - keypair generation, sign / verify, and on-the-wire base64 / JWK formats
//!   round-trip across module boundaries
//! - ClotoHub's "issue once, verify offline" workflow works as advertised
//! - the HMAC API from v0.1.x is untouched and still imports cleanly alongside
//!   the new Ed25519 surface

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use mgp_seal::ed25519::{
    generate_keypair, private_key_from_base64, public_key_to_jwk, sign, verify, KeyId, PublicKey,
    Signature, PUBLIC_KEY_LENGTH,
};
use mgp_seal::{compute_seal, verify_seal};
use rand::rngs::OsRng;

const SEAL_HMAC_KEY: &[u8] = b"hmac-key-for-tier1-coexistence-test!";

#[test]
fn issue_once_verify_offline_workflow() {
    // Issuer side (ClotoHub): generate a master keypair, publish the public
    // key out-of-band, sign a canonical seal payload.
    let (issuer_priv, issuer_pub) = generate_keypair(&mut OsRng);
    let kid = KeyId::new("clotohub-master-v1").unwrap();
    let canonical = b"connector=cpersona\nversion=2.4.12\nentry_point_sha256=deadbeef\n";
    let sig = sign(&issuer_priv, &kid, canonical);

    // Verifier side (ClotoCore): receives only the issuer's published public
    // key (via base64 from a checked-in `.pub` file or DNS TXT) and the seal's
    // base64 signature payload + key_id. No private key, no oracle.
    let pub_b64 = issuer_pub.to_base64();
    let sig_b64 = sig.to_base64();

    let received_pub = PublicKey::from_base64(&pub_b64).unwrap();
    let received_sig = Signature::from_base64(&sig_b64).unwrap();

    assert!(verify(&received_pub, &kid, canonical, &received_sig));
}

#[test]
fn jwks_x_field_decodes_to_public_key() {
    // A real JWKS consumer reads the `x` field, base64url-no-pad decodes it,
    // and reconstructs a usable verifying key. This test enforces that
    // round-trip end-to-end.
    let (_, pk) = generate_keypair(&mut OsRng);
    let kid = KeyId::new("clotohub-master-v1").unwrap();
    let jwk = public_key_to_jwk(&pk, &kid);

    let x = jwk["x"].as_str().expect("`x` must be a string");
    let raw = URL_SAFE_NO_PAD
        .decode(x)
        .expect("`x` decodes as base64url-no-pad");
    let arr: [u8; PUBLIC_KEY_LENGTH] = raw.try_into().expect("`x` decodes to exactly 32 bytes");
    assert_eq!(arr, pk.to_bytes());
}

#[test]
fn private_key_env_var_roundtrip() {
    // Models the production deployment path where ClotoHub reads its master
    // private key from CLOTO_SEAL_ED25519_PRIVATE_KEY (base64) at boot.
    let (sk, pk) = generate_keypair(&mut OsRng);
    let kid = KeyId::new("clotohub-master-v1").unwrap();
    let env_value = sk.to_base64();

    let restored_sk = private_key_from_base64(&env_value).unwrap();
    let sig = sign(&restored_sk, &kid, b"deployment payload");
    assert!(verify(&pk, &kid, b"deployment payload", &sig));
}

#[test]
fn rebinding_signature_to_different_kid_fails() {
    // Defence-in-depth: even if an attacker controls JWKS and relabels a
    // signature with a different `kid`, verification with the new kid must
    // fail because kid is mixed into the signing input.
    let (sk, pk) = generate_keypair(&mut OsRng);
    let kid_real = KeyId::new("clotohub-master-v1").unwrap();
    let kid_attacker = KeyId::new("clotohub-master-evil").unwrap();
    let payload = b"important seal";

    let sig = sign(&sk, &kid_real, payload);
    assert!(verify(&pk, &kid_real, payload, &sig));
    assert!(!verify(&pk, &kid_attacker, payload, &sig));
}

#[test]
fn hmac_api_still_works_alongside_ed25519() {
    // Tier 1 (HMAC) and Tier 2 (Ed25519) co-exist in the same crate; this
    // test exercises both via the public API to catch any accidental
    // breakage when bumping consumer Cargo.toml from 0.1.x to 0.2.0.
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), b"tier1+tier2 coexistence").unwrap();

    let hmac_seal = compute_seal(tmp.path(), SEAL_HMAC_KEY).unwrap();
    assert!(verify_seal(tmp.path(), &hmac_seal, SEAL_HMAC_KEY).unwrap());

    let (sk, pk) = generate_keypair(&mut OsRng);
    let kid = KeyId::new("clotohub-master-v1").unwrap();
    let ed_sig = sign(&sk, &kid, hmac_seal.as_bytes());
    assert!(verify(&pk, &kid, hmac_seal.as_bytes(), &ed_sig));
}
