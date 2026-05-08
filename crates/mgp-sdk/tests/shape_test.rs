//! Shape conversion tests: `ConnectorManifest` → `RegistryEntry`.

use mgp_sdk::adapters::{GitSpec, SourceSpec};
use mgp_sdk::shape::{manifest_to_registry_entry, RegistryEntry};
use mgp_sdk::types::{ConnectorManifest, EnvVarDef, InstallSpec};

fn manifest_with_full_optional_fields() -> ConnectorManifest {
    ConnectorManifest {
        spec_version: 1,
        connector_type: "mgp_server".to_string(),
        id: "shape-demo".to_string(),
        name: "Shape Demo".to_string(),
        description: "shape conversion exhibit".to_string(),
        version: "1.2.3".to_string(),
        category: "demo".to_string(),
        trust_level: "experimental".to_string(),
        magic_seal: "sha256:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
            .to_string(),
        install: InstallSpec {
            source: SourceSpec::Git(GitSpec {
                url: "https://github.com/Cloto-dev/x.git".to_string(),
                reference: "v1.2.3".to_string(),
                subdir: None,
            }),
            package_manager: "uv".to_string(),
            runtime: "rust".to_string(),
            dependencies: vec!["dep-a".to_string(), "dep-b".to_string()],
            directory: "x".to_string(),
            bin_name: Some("xbin".to_string()),
        },
        icon: Some("icon.svg".to_string()),
        tags: vec!["t1".to_string()],
        host_compatibility: vec!["clotocore".to_string()],
        env_vars: vec![EnvVarDef {
            name: "X_API_KEY".to_string(),
            description: Some("API key".to_string()),
        }],
        optional_env_vars: vec![EnvVarDef {
            name: "X_DEBUG".to_string(),
            description: None,
        }],
        auto_restart: true,
        changelog: Some("v1.2.3 release".to_string()),
    }
}

#[test]
fn manifest_to_registry_entry_preserves_all_fields() {
    let m = manifest_with_full_optional_fields();
    let entry = manifest_to_registry_entry(&m);

    assert_eq!(entry.id, m.id);
    assert_eq!(entry.name, m.name);
    assert_eq!(entry.description, m.description);
    assert_eq!(entry.category, m.category);
    assert_eq!(entry.version, m.version);
    assert_eq!(entry.directory, m.install.directory);
    assert_eq!(entry.dependencies, m.install.dependencies);
    assert_eq!(entry.env_vars, m.env_vars);
    assert_eq!(entry.optional_env_vars, m.optional_env_vars);
    assert_eq!(entry.tags, m.tags);
    assert_eq!(entry.trust_level, m.trust_level);
    assert_eq!(entry.auto_restart, m.auto_restart);
    assert_eq!(entry.icon, m.icon);
    assert_eq!(entry.runtime, m.install.runtime);
    assert_eq!(entry.bin_name, m.install.bin_name);
    assert_eq!(entry.changelog, m.changelog);
    assert_eq!(entry.seal.as_deref(), Some(m.magic_seal.as_str()));
}

#[test]
fn registry_entry_round_trips_through_json() {
    let entry = manifest_to_registry_entry(&manifest_with_full_optional_fields());
    let json = serde_json::to_string(&entry).expect("serialize");
    let again: RegistryEntry = serde_json::from_str(&json).expect("parse");
    assert_eq!(entry, again);
}

#[test]
fn registry_entry_omitted_optional_fields_use_defaults_on_parse() {
    // ClotoCore historically tolerates missing optional fields.
    // Confirm the shape stays compatible.
    let minimal = r#"{
        "id": "x",
        "name": "X",
        "description": "x",
        "category": "c",
        "version": "0.1.0"
    }"#;
    let entry: RegistryEntry = serde_json::from_str(minimal).expect("parse minimal");
    assert_eq!(entry.id, "x");
    assert_eq!(entry.trust_level, "standard"); // default
    assert_eq!(entry.runtime, "python"); // default
    assert!(entry.seal.is_none());
}
