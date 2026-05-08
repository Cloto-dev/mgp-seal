# Changelog

All notable changes to `mgp-seal` are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning: [SemVer](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/Cloto-dev/mgp-seal/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Cloto-dev/mgp-seal/releases/tag/v0.1.0
