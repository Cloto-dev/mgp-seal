# mgp-rs Development Rules

Rust workspace hosting reference implementations for **MGP** (see [`mgp-spec`](https://github.com/Cloto-dev/mgp-spec) for the canonical specification). Two crates: `mgp-seal` (§8 L0 Magic Seal — HMAC-SHA256 Tier 1 + Ed25519 Tier 2) and `mgp-sdk` (`cloto-connector.json` v1 types, validation, source-spec adapters, registry shape).

> Inherits: `../CLAUDE.md` — Conventions (RFC 2119), Public Repository English-only, Git Rules.

## Mandatory Reads

- **`../mgp-spec/docs/MGP_SPEC.md` + `../mgp-spec/docs/MGP_CONNECTOR.md`** — Protocol authority. `mgp-rs` is a downstream consumer of these specs.
- **`../mgp-spec/schemas/connector/v1.json`** — JSON Schema authority that `mgp-sdk/src/validate_v1.rs` must conform to.

## Workspace Layout

| Crate | Path | Role |
|---|---|---|
| `mgp-seal` | `crates/mgp-seal/` | Reference impl of §8 L0 Magic Seal (HMAC-SHA256 Tier 1 + Ed25519 Tier 2) |
| `mgp-sdk`  | `crates/mgp-sdk/`  | `cloto-connector.json` v1 types, validation, source-spec adapters, registry shape |

**Workspace inheritance** (root `Cargo.toml`):
- `edition = "2021"`, `rust-version = "1.78"`, `license = "MIT"`, `repository`, `homepage` — inherited via `.workspace = true`.
- `version` is per-crate; touch only the bumped crate's `[package]` section.

## Commands

- Build: `cargo build`
- Test: `cargo test --all-targets`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Format check: `cargo fmt --all -- --check`
- Format: `cargo fmt --all`

CI (`.github/workflows/ci.yml`) runs `test` / `clippy` / `fmt --check` on push to `main` and on every PR.

## Tag Convention

- **Going forward**: `mgp-{crate-name}-vX.Y.Z` (e.g. `mgp-seal-v0.2.0`, `mgp-sdk-v0.1.2`).
- **Historical**: bare `v0.1.0` / `v0.2.0` predate the workspace conversion (2026-05-08 Phase 5a rename from `mgp-seal` to `mgp-rs`). Preserved for redirect compatibility with downstream pins.

**MUST**: Each release commit mints one tag per crate that was version-bumped (two crates released in the same commit ⇒ two tags).

## Cargo Tag Dedup Gotcha (`feedback_cargo_workspace_tag_dedup`)

Cargo treats `?tag=A` and `?tag=B` as **different sources** even when they resolve to the same SHA. A downstream `Cargo.lock` can carry two entries for the same crate.

**MUST**: Before shipping a tag bump that closes the gap, run `grep -c '<crate-name>' Cargo.lock` in every known consumer and confirm exactly one entry. If two appear, eliminate the dead pin upstream **before** publishing the new tag.

## SDK Leniency Taxonomy (`feedback_sdk_leniency_taxonomy`)

When the SDK is more lenient than the spec, distinguish:

- **(α) Implementation-cost leniency** — types, derives, helpers, better errors. Genuine user-friendliness.
- **(β) Acceptance leniency** — accepts inputs the spec rejects (e.g. uppercase hex when the spec mandates lowercase). **Silent-bug pattern, not user-friendliness.**

**MUST**: Strict-by-default validators reject (β). If a lenient mode is genuinely desired, ship it as an explicit opt-in API (`validate_v1_lenient()`, etc.) — never by softening the strict one. The `mgp-sdk-v0.1.2` patch (`c5bde1e`) tightened `magic_seal` validation from `is_ascii_hexdigit` to lowercase-only after recognizing the uppercase-accepting form was (β).

## Workspace Inheritance Discipline

- **MUST**: Use `field.workspace = true` for `edition`, `rust-version`, `license`, `repository`, `homepage` in every member crate.
- **MUST NOT**: Bump `edition` / `rust-version` / `license` in a single crate's `[package]` section — if change is warranted, update the workspace root and accept it across all crates.
- **SHOULD**: Place shared dependency versions in `[workspace.dependencies]` and reference via `.workspace = true` in member crates.

## Consuming `mgp-rs` from Downstream Crates

Recommended pin style (substitute current tags from `git tag -l`):

```toml
[dependencies]
mgp-seal = { git = "https://github.com/Cloto-dev/mgp-rs", subdir = "crates/mgp-seal", tag = "mgp-seal-vX.Y.Z" }
mgp-sdk  = { git = "https://github.com/Cloto-dev/mgp-rs", subdir = "crates/mgp-sdk",  tag = "mgp-sdk-vX.Y.Z" }
```

The historical bare-tag pin (`tag = "v0.2.0"`) still works via GitHub auto-redirect but **SHOULD** be migrated to the namespaced form in any new code.

## Public Repo Implications

This repo is `visibility=public` and MIT-licensed. Per the parent rule:

- All Markdown, doc comments, `cargo` field strings, commit messages, and PR descriptions **MUST** be English.

## Prohibited

- **MUST NOT**: Place protocol normative text in this repo — that lives in [`mgp-spec`](https://github.com/Cloto-dev/mgp-spec).
- **MUST NOT**: Bypass `cargo fmt` / `cargo clippy -- -D warnings` via `--no-verify` push. Fix the violation; CI will catch otherwise.
- **SHOULD NOT**: Bundle `cargo update` / `Cargo.lock` churn into unrelated PRs — keep dep updates reviewable in their own commit.
