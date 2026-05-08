# mgp-sdk

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Pure-logic SDK for MGP connectors. Defines the Rust types,
validation, source-spec adapters, and registry shape conversion that
both ClotoHub.dev's catalog sync worker and ClotoCore's install path
need to handle a `cloto-connector.json` v1 manifest.

The crate is intentionally pure (no `tokio`, no `reqwest`) so it can
be reused on both sides without forcing either to adopt unwanted
dependencies. Network IO is the consumer's concern.

## Feature scope (v0.1.0)

- `types` — Rust definitions for the v1 manifest schema.
- `validate` — declarative validation (`spec_version`, `connector_type`,
  `magic_seal`, `package_manager`, `runtime`, `trust_level`, source
  sub-spec).
- `adapters` — typed wrappers over the four v1 source kinds:
  - `git`
  - `raw_url`
  - `pypi`
  - `docker`
- `shape` — conversion from manifest to `registry.json` entry shape
  ClotoCore consumes today.

Out of scope for v0.1.0: `hf_hub`, `npm`, `ipfs`, `custom` adapters
(deferred to Phase 6+), and the `cloto-connector.json` formal JSON
schema document (lives in `mgp-spec`, this crate is a consumer).

## Relation to `mgp-seal`

Sibling crate in the same `mgp-rs` workspace. A future `mgp-sdk`
release (tracked for v0.2.0) will depend on `mgp-seal` to expose
Magic Seal verification helpers; v0.1.x deliberately ships without
that dependency so consumers that only need the manifest types and
validation layer get a minimal dependency graph. Versioned
independently per crate (tag namespaces: `mgp-seal-vX.Y.Z`,
`mgp-sdk-vX.Y.Z`).

## License

MIT — see [LICENSE](../../LICENSE).
