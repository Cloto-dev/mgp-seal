//! `pypi` source adapter.

use serde::{Deserialize, Serialize};

/// PyPI package. Consumer is expected to materialize via
/// `uv pip install <package>[==<version>]`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PypiSpec {
    /// Package name as it appears on PyPI.
    pub package: String,
    /// Optional version pin (PEP 440). Empty / `None` allows latest.
    #[serde(default)]
    pub version: Option<String>,
}

impl PypiSpec {
    /// Render the dependency in `uv pip install` argument form.
    /// `pkg` for unpinned, `pkg==X.Y.Z` for pinned.
    #[must_use]
    pub fn install_argument(&self) -> String {
        match &self.version {
            Some(v) if !v.is_empty() => format!("{}=={}", self.package, v),
            _ => self.package.clone(),
        }
    }

    /// Light validation of the package name. Pure logic only.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the package name is empty or contains
    /// whitespace.
    pub fn check(&self) -> Result<(), &'static str> {
        if self.package.is_empty() {
            return Err("pypi package is empty");
        }
        if self.package.chars().any(char::is_whitespace) {
            return Err("pypi package contains whitespace");
        }
        Ok(())
    }
}
