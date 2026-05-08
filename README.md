# mgp-seal

Reference implementation of **MGP §8 L0 Magic Seal** — HMAC-SHA256 binary
integrity verification for the [Magic Gateway Protocol](https://github.com/Cloto-dev/mgp-spec).

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

Magic Seal is the cryptographic primitive that backs MGP's binary integrity
guarantee. Each MGP server binary is sealed with an HMAC-SHA256 computed over
its file contents using a per-installation key. At startup the kernel verifies
the seal and translates the result into a trust-level decision per
[MGP §4.0](https://github.com/Cloto-dev/mgp-spec) (the v0.6.3 behavior matrix).

This crate is the canonical reference implementation, extracted from
[ClotoCore](https://github.com/Cloto-dev/ClotoCore) so any MGP-compatible
runtime — kernel, registry, or verification tooling — can share the same
cryptographic primitives.

## Install

```toml
[dependencies]
mgp-seal = { git = "https://github.com/Cloto-dev/mgp-seal", tag = "v0.1.0" }
```

## Quick start

```rust
use std::path::Path;
use mgp_seal::{compute_seal, verify_seal, load_or_generate_seal_key};

fn main() -> anyhow::Result<()> {
    let key = load_or_generate_seal_key(Path::new("/var/lib/mgp"))?;
    let seal = compute_seal(Path::new("/usr/local/bin/my-mgp-server"), &key)?;
    assert!(verify_seal(
        Path::new("/usr/local/bin/my-mgp-server"),
        &seal,
        &key,
    )?);
    Ok(())
}
```

## API

| Function / Type | Purpose |
|---|---|
| `compute_seal(file, key) -> "sha256:HEX"` | HMAC-SHA256 over file contents |
| `verify_seal(file, expected, key) -> bool` | Constant-time check against a stored seal |
| `check_seal(trust_level, seal, file, key, allow_unsigned)` | Full §4.0 behavior matrix |
| `SealStatus` | `Verified` / `Failed` / `Unsigned` / `Skipped` |
| `TrustLevel` | `Untrusted < Experimental < Standard < Core` (§2) |
| `load_or_generate_seal_key(data_dir)` | Env → file → fresh-random fallback |

## Seal absent: §4.0 (v0.6.3) behavior matrix

| Trust Level   | Seal Present | Seal Absent       | Seal Invalid |
|---------------|--------------|-------------------|--------------|
| Core          | Verify       | Force `untrusted` | Block        |
| Standard      | Verify       | Force `untrusted` | Block        |
| Experimental  | Verify       | Force `untrusted` | Block        |
| Untrusted     | Verify       | Allow             | Block        |

`check_seal` only computes the cryptographic outcome — translating the
result into a startup decision (block / start untrusted / start declared)
is the caller's responsibility. See the [doc comment](src/seal.rs) for
audit-event semantics.

## Key resolution

`load_or_generate_seal_key(data_dir)` resolves in this order:

1. `CLOTO_SEAL_KEY` env-var (hex-encoded). The name is preserved from the
   ClotoCore reference; other runtimes MAY adopt it for portability or wrap
   this function with a runtime-specific env-var.
2. `{data_dir}/seal.key` (raw bytes, 32 bytes recommended).
3. Generate a fresh 32-byte random key, persist it to `{data_dir}/seal.key`,
   return it.

## Versioning & stability

Pre-1.0 the API may change between minor versions. The cryptographic
contract — `"sha256:HEX"` format and the §4.0 behavior matrix — is
stable per the MGP spec and will not change without a corresponding
spec bump.

## License

MIT — see [LICENSE](LICENSE).
