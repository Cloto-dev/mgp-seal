//! Round-trip serde tests for [`ConnectorManifest`].

use mgp_sdk::adapters::SourceSpec;
use mgp_sdk::ConnectorManifest;

const SAMPLE_MANIFEST: &str = r#"{
    "spec_version": 1,
    "connector_type": "mgp_server",
    "id": "demo-server",
    "name": "Demo",
    "description": "demo",
    "version": "0.1.0",
    "category": "test",
    "trust_level": "standard",
    "magic_seal": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
    "install": {
        "source": {
            "type": "git",
            "url": "https://github.com/Cloto-dev/demo.git",
            "reference": "v0.1.0"
        },
        "package_manager": "uv",
        "runtime": "python",
        "directory": "demo"
    },
    "tags": ["demo"],
    "host_compatibility": ["clotocore"]
}"#;

#[test]
fn round_trip_preserves_known_fields() {
    let m: ConnectorManifest = serde_json::from_str(SAMPLE_MANIFEST).expect("parse");
    assert_eq!(m.id, "demo-server");
    assert_eq!(m.spec_version, 1);
    assert!(matches!(m.install.source, SourceSpec::Git(_)));
    let again = serde_json::to_string(&m).expect("serialize");
    let m2: ConnectorManifest = serde_json::from_str(&again).expect("re-parse");
    assert_eq!(m, m2);
}

#[test]
fn unknown_top_level_fields_are_ignored() {
    let with_extra = SAMPLE_MANIFEST.replace(
        "\"host_compatibility\": [\"clotocore\"]",
        "\"host_compatibility\": [\"clotocore\"], \"future_top_level\": \"ignored\"",
    );
    let m: ConnectorManifest = serde_json::from_str(&with_extra).expect("parse with extra");
    assert_eq!(m.id, "demo-server");
}

#[test]
fn each_source_kind_round_trips() {
    let kinds = [
        (
            r#"{"type":"git","url":"https://example.com/x.git","reference":"main"}"#,
            "git",
        ),
        (
            r#"{"type":"raw_url","url":"https://example.com/x.tar.gz"}"#,
            "raw_url",
        ),
        (r#"{"type":"pypi","package":"demo"}"#, "pypi"),
        (
            r#"{"type":"docker","image":"nginx","tag":"1.27"}"#,
            "docker",
        ),
    ];
    for (json, expected_kind) in kinds {
        let s: SourceSpec = serde_json::from_str(json).expect(json);
        assert_eq!(s.kind(), expected_kind);
        let again = serde_json::to_string(&s).expect("serialize");
        let s2: SourceSpec = serde_json::from_str(&again).expect("re-parse");
        assert_eq!(s, s2);
    }
}
