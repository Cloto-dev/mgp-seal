//! [`TrustLevel`] — MGP §2 server trust classification.

use serde::{Deserialize, Serialize};

/// Server trust level — determined by the kernel (mcp.toml config), not self-declared.
///
/// Ordering: `Untrusted < Experimental < Standard < Core`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrustLevel {
    /// Lowest tier — sandbox by default, no privileged capabilities.
    Untrusted,
    /// Experimental servers — limited capabilities, dev-mode friendly.
    Experimental,
    /// Standard third-party servers — typical capability set.
    Standard,
    /// First-party / vetted servers — full capability set.
    Core,
}

impl TrustLevel {
    /// Parse a trust level string. Unknown values map to [`TrustLevel::Untrusted`]
    /// (fail-closed default per MGP §4.0).
    #[must_use]
    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "core" => Self::Core,
            "standard" => Self::Standard,
            "experimental" => Self::Experimental,
            _ => Self::Untrusted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_is_strict() {
        assert!(TrustLevel::Untrusted < TrustLevel::Experimental);
        assert!(TrustLevel::Experimental < TrustLevel::Standard);
        assert!(TrustLevel::Standard < TrustLevel::Core);
    }

    #[test]
    fn from_str_lossy_known() {
        assert_eq!(TrustLevel::from_str_lossy("core"), TrustLevel::Core);
        assert_eq!(TrustLevel::from_str_lossy("standard"), TrustLevel::Standard);
        assert_eq!(
            TrustLevel::from_str_lossy("experimental"),
            TrustLevel::Experimental
        );
        assert_eq!(
            TrustLevel::from_str_lossy("untrusted"),
            TrustLevel::Untrusted
        );
    }

    #[test]
    fn from_str_lossy_unknown_is_untrusted() {
        assert_eq!(TrustLevel::from_str_lossy("unknown"), TrustLevel::Untrusted);
        assert_eq!(TrustLevel::from_str_lossy(""), TrustLevel::Untrusted);
    }

    #[test]
    fn from_str_lossy_case_insensitive() {
        assert_eq!(TrustLevel::from_str_lossy("CORE"), TrustLevel::Core);
        assert_eq!(TrustLevel::from_str_lossy("Standard"), TrustLevel::Standard);
    }

    #[test]
    fn serde_lowercase() {
        let core = TrustLevel::Core;
        let json = serde_json::to_string(&core).unwrap();
        assert_eq!(json, "\"core\"");

        let parsed: TrustLevel = serde_json::from_str("\"untrusted\"").unwrap();
        assert_eq!(parsed, TrustLevel::Untrusted);
    }
}
