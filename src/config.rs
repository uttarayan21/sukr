//! Site configuration loading.

use crate::error::{Error, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Site-wide configuration loaded from site.toml.
#[derive(Debug, Deserialize)]
pub struct SiteConfig {
    /// Site title (used in page titles and nav).
    pub title: String,
    /// Site author name.
    pub author: String,
    /// Base URL for the site (used for feeds, canonical links).
    pub base_url: String,
}

impl SiteConfig {
    /// Load configuration from a TOML file.
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(|e| Error::ReadFile {
            path: path.to_path_buf(),
            source: e,
        })?;

        toml::from_str(&content).map_err(|e| Error::Config {
            path: path.to_path_buf(),
            message: e.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let toml = r#"
            title = "Test Site"
            author = "Test Author"
            base_url = "https://example.com/"
        "#;

        let config: SiteConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.title, "Test Site");
        assert_eq!(config.author, "Test Author");
        assert_eq!(config.base_url, "https://example.com/");
    }
}
