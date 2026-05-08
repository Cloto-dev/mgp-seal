# Changelog

All notable changes to `mgp-seal` are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning: [SemVer](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Repository renamed `Cloto-dev/mgp-seal` → `Cloto-dev/mgp-rs` and converted
  to a `cargo workspace`. Sources moved from the repo root to
  `crates/mgp-seal/`. Existing `v0.1.0` and `v0.2.0` tags continue to point
  at the pre-rename top-level layout — consumers pinning either tag keep
  working unchanged via GitHub auto-redirect.
- Future releases use the per-crate tag namespace `mgp-seal-vX.Y.Z`.

## [0.2.0] - 2026-05-08

### Added

- New `mgp_seal::ed25519` module — Tier 2 of the Magic Seal trust model,
  asymmetric signing alongside the existing HMAC oracle so verifiers can
  validate seals **offline** with only the issuer's public key:
  - `generate_keypair(rng) -> (PrivateKey, PublicKey)`
  - `sign(&PrivateKey, &KeyId, message) -> Signature`
  - `verify(&PublicKey, &KeyId, message, &Signature) -> bool`
  - `public_key_to_jwk(&PublicKey, &KeyId)` — RFC 8037 §2 OKP/Ed25519 JWK
  - `private_key_from_base64(s)` — 12-factor env-var loading (pairs with
    `PrivateKey::to_base64` for round-trip storage in
    `CLOTO_SEAL_ED25519_PRIVATE_KEY` etc.)
  - `PublicKey::{to_base64, from_base64}`,
    `Signature::{to_base64, from_base64}` — stable wire formats
  - `KeyId::new(s)` — stable, length-bounded keypair identifier
- All Ed25519 signatures are computed over the domain-separated input
  `b"mgp-seal-ed25519-v1" || 0x00 || key_id || 0x00 || message` so a
  signature cannot be reused across protocols or silently reattributed to a
  different `kid` in a JWKS keyring. Bumping the `-v1` suffix is reserved
  for a future major version.
- 15 module-level unit tests + 5 integration tests covering keygen, sign /
  verify round-trip, tampered-message rejection, kid-rebinding rejection,
  determinism, JWK shape, base64 round-trip for keys and signatures, and
  Tier 1 + Tier 2 coexistence.

### Changed

- `serde_json` is now a regular dependency (was dev-only) — used by
  `public_key_to_jwk` to return a `serde_json::Value`.
- `ed25519-dalek = "2"` and `base64 = "0.22"` added as new dependencies.
- README expanded with a Tier 2 API table and quick-start; install snippet
  bumped to `tag = "v0.2.0"`.

### Notes

- **Backwards compatible.** The Tier 1 HMAC API (`compute_seal`,
  `verify_seal`, `check_seal`, `SealStatus`, `TrustLevel`,
  `load_or_generate_seal_key`) is byte-for-byte identical to v0.1.0 — no
  consumer needs to change call sites when bumping. ClotoCore can continue
  using the v0.1.x HMAC path until it opts into Ed25519 verification.
- The Ed25519 surface is intentionally namespaced (`mgp_seal::ed25519::*`)
  rather than re-exported at the crate root, so call sites read clearly
  next to the HMAC API.

## [0.1.0] - 2026-05-08

### Added

- Initial extraction from `ClotoCore/crates/core/src/managers/mcp_seal.rs` (513 lines)
  as the canonical reference implementation of MGP §8 L0 Magic Seal.
- Public API:
  - `compute_seal(file_path, key) -> "sha256:HEX"`
  - `verify_seal(file_path, expected, key) -> bool` (constant-time)
  - `check_seal(trust_level, seal_value, entry_point, key, allow_unsigned) -> SealStatus`
    (implements the MGP §4.0 v0.6.3 behavior matrix)
  - `load_or_generate_seal_key(data_dir)` (env → file → fresh-random fallback)
- Public types: `SealStatus`, `TrustLevel`.
- Integration test suite covering compute/verify, the full §4.0 behavior matrix,
  and key resolution.

### Notes

- Cryptographic behavior is identical to the ClotoCore source; this is a
  refactor extraction, not a redesign. Consumers (e.g. ClotoCore, ClotoHub.dev)
  can swap in this crate without observable runtime change.

[Unreleased]: https://github.com/Cloto-dev/mgp-seal/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Cloto-dev/mgp-seal/releases/tag/v0.2.0
[0.1.0]: https://github.com/Cloto-dev/mgp-seal/releases/tag/v0.1.0
