//! Content discovery and frontmatter parsing.

use crate::error::{Error, Result};
use gray_matter::{engine::YAML, Matter};
use std::fs;
use std::path::{Path, PathBuf};

/// Parsed frontmatter from a content file.
#[derive(Debug)]
pub struct Frontmatter {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub tags: Vec<String>,
}

/// A content item ready for rendering.
#[derive(Debug)]
pub struct Content {
    pub frontmatter: Frontmatter,
    pub body: String,
    pub source_path: PathBuf,
    pub slug: String,
}

impl Content {
    /// Load and parse a markdown file with TOML frontmatter.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let raw = fs::read_to_string(path).map_err(|e| Error::ReadFile {
            path: path.to_path_buf(),
            source: e,
        })?;

        let matter = Matter::<YAML>::new();
        let parsed = matter.parse(&raw);

        let frontmatter = parse_frontmatter(path, &parsed)?;

        // Derive slug from filename (without extension)
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_string();

        Ok(Content {
            frontmatter,
            body: parsed.content,
            source_path: path.to_path_buf(),
            slug,
        })
    }

    /// Compute the output path relative to the output directory.
    /// e.g., content/blog/foo.md → blog/foo/index.html
    pub fn output_path(&self, content_root: &Path) -> PathBuf {
        let relative = self
            .source_path
            .strip_prefix(content_root)
            .unwrap_or(&self.source_path);

        let parent = relative.parent().unwrap_or(Path::new(""));
        parent.join(&self.slug).join("index.html")
    }
}

fn parse_frontmatter(path: &Path, parsed: &gray_matter::ParsedEntity) -> Result<Frontmatter> {
    let data = parsed.data.as_ref().ok_or_else(|| Error::Frontmatter {
        path: path.to_path_buf(),
        message: "missing frontmatter".to_string(),
    })?;

    let pod = data.as_hashmap().map_err(|_| Error::Frontmatter {
        path: path.to_path_buf(),
        message: "frontmatter is not a valid map".to_string(),
    })?;

    let title = pod
        .get("title")
        .and_then(|v| v.as_string().ok())
        .ok_or_else(|| Error::Frontmatter {
            path: path.to_path_buf(),
            message: "missing required 'title' field".to_string(),
        })?;

    let description = pod.get("description").and_then(|v| v.as_string().ok());
    let date = pod.get("date").and_then(|v| v.as_string().ok());

    // Handle nested taxonomies.tags structure
    let tags = if let Some(taxonomies) = pod.get("taxonomies") {
        if let Ok(tax_map) = taxonomies.as_hashmap() {
            if let Some(tags_pod) = tax_map.get("tags") {
                if let Ok(tags_vec) = tags_pod.as_vec() {
                    tags_vec.iter().filter_map(|v| v.as_string().ok()).collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Ok(Frontmatter {
        title,
        description,
        date,
        tags,
    })
}
