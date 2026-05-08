//! Per-installation seal key management.

use std::path::Path;

use anyhow::{bail, Context};
use rand::Rng;

/// Load or generate the per-installation seal key.
///
/// Resolution order:
/// 1. `CLOTO_SEAL_KEY` environment variable (hex-encoded).
/// 2. `{data_dir}/seal.key` file (raw bytes).
/// 3. Generate a new random 32-byte key, save to `{data_dir}/seal.key`.
///
/// The `CLOTO_SEAL_KEY` env-var name is preserved from the ClotoCore reference
/// implementation. Other MGP runtimes MAY adopt the same name to ease key
/// portability across kernels, or wrap this function to read a runtime-specific
/// env-var.
pub fn load_or_generate_seal_key(data_dir: &Path) -> anyhow::Result<Vec<u8>> {
    // 1. Check environment variable.
    if let Ok(env_key) = std::env::var("CLOTO_SEAL_KEY") {
        let key = hex::decode(env_key.trim())
            .context("CLOTO_SEAL_KEY environment variable is not valid hex")?;
        if key.is_empty() {
            bail!("CLOTO_SEAL_KEY environment variable is empty");
        }
        return Ok(key);
    }

    // 2. Check existing key file.
    let key_path = data_dir.join("seal.key");
    if key_path.exists() {
        let key = std::fs::read(&key_path)
            .with_context(|| format!("Failed to read seal key: {}", key_path.display()))?;
        if key.is_empty() {
            bail!("Seal key file exists but is empty: {}", key_path.display());
        }
        return Ok(key);
    }

    // 3. Generate new key.
    let mut rng = rand::thread_rng();
    let mut key = vec![0u8; 32];
    rng.fill(&mut key[..]);

    // Ensure data directory exists.
    std::fs::create_dir_all(data_dir)
        .with_context(|| format!("Failed to create data directory: {}", data_dir.display()))?;

    std::fs::write(&key_path, &key)
        .with_context(|| format!("Failed to write seal key: {}", key_path.display()))?;

    Ok(key)
}
