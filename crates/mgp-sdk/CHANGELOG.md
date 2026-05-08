# Changelog

All notable changes to `mgp-sdk` are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning: [SemVer](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-05-08

### Added

- Initial release. Pure-logic SDK for the `cloto-connector.json` v1
  manifest, intentionally no `tokio` / no `reqwest` so the same crate
  is reusable from both ClotoHub.dev's catalog sync worker and
  ClotoCore's install path.
- `types::ConnectorManifest` — Rust definitions for the v1 schema
  with forward-compat `#[serde(default)]` and unknown-field tolerance.
- `validate::validate_v1` — declarative checks for `spec_version`,
  `connector_type`, kebab-case `id`, MGP §2.3 `trust_level`,
  Magic Seal format (`sha256:HEX`), `package_manager == "uv"`,
  `runtime ∈ {python, rust, node}`, and source sub-spec.
- `adapters::{GitSpec, RawUrlSpec, PypiSpec, DockerSpec}` — typed
  wrappers over the four v1 source kinds, each with a pure
  `check()` validator. `SourceSpec::kind()` exposes the JSON tag.
- `shape::manifest_to_registry_entry` — conversion from a manifest
  into a `registry.json` entry that ClotoCore consumes today, plus
  a [`RegistryEntry`] type with field layout mirroring
  `cloto_core::handlers::marketplace::RegistryEntry` byte-for-byte.

### Out of scope (Phase 6+ candidates)

- `hf_hub`, `npm`, `ipfs`, `custom` source adapters.
- A formal JSON Schema document for `cloto-connector.json` (lives
  in `mgp-spec`; this crate is a consumer).
- Network IO (delegated to consumers — see `project_clotohub_phase_5_design.md`
  §4-B on the pure-logic decision).
