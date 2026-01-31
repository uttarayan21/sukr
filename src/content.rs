//! Content discovery and frontmatter parsing.

use crate::error::{Error, Result};
use gray_matter::{engine::YAML, Matter};
use std::fs;
use std::path::{Path, PathBuf};

/// The type of content being processed.
#[derive(Debug, Clone, PartialEq)]
pub enum ContentKind {
    /// Blog post with full metadata (date, tags, etc.)
    Post,
    /// Standalone page (about, collab)
    Page,
    /// Section index (_index.md)
    Section,
    /// Project card with external link
    Project,
}

/// A navigation menu item discovered from the filesystem.
#[derive(Debug, Clone)]
pub struct NavItem {
    /// Display label (from nav_label or title)
    pub label: String,
    /// URL path (e.g., "/blog/index.html" or "/about.html")
    pub path: String,
    /// Sort order (lower = first, default 50)
    pub weight: i64,
}

/// Parsed frontmatter from a content file.
#[derive(Debug)]
pub struct Frontmatter {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub tags: Vec<String>,
    /// Sort order for nav and listings
    pub weight: Option<i64>,
    /// For project cards: external link
    pub link_to: Option<String>,
    /// Custom navigation label (defaults to title)
    pub nav_label: Option<String>,
}

/// A content item ready for rendering.
#[derive(Debug)]
pub struct Content {
    pub kind: ContentKind,
    pub frontmatter: Frontmatter,
    pub body: String,
    pub source_path: PathBuf,
    pub slug: String,
}

impl Content {
    /// Load and parse a markdown file with YAML frontmatter.
    pub fn from_path(path: impl AsRef<Path>, kind: ContentKind) -> Result<Self> {
        Self::from_path_inner(path.as_ref(), kind)
    }

    fn from_path_inner(path: &Path, kind: ContentKind) -> Result<Self> {
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
            kind,
            frontmatter,
            body: parsed.content,
            source_path: path.to_path_buf(),
            slug,
        })
    }

    /// Compute the output path relative to the output directory.
    /// e.g., content/blog/foo.md → blog/foo.html
    pub fn output_path(&self, content_root: &Path) -> PathBuf {
        let relative = self
            .source_path
            .strip_prefix(content_root)
            .unwrap_or(&self.source_path);

        match self.kind {
            ContentKind::Section => {
                // _index.md → parent/index.html (listing pages stay as index.html)
                let parent = relative.parent().unwrap_or(Path::new(""));
                parent.join("index.html")
            }
            _ => {
                // Regular content → parent/slug.html (flat structure)
                let parent = relative.parent().unwrap_or(Path::new(""));
                parent.join(format!("{}.html", self.slug))
            }
        }
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
    let weight = pod.get("weight").and_then(|v| v.as_i64().ok());
    let link_to = pod.get("link_to").and_then(|v| v.as_string().ok());
    let nav_label = pod.get("nav_label").and_then(|v| v.as_string().ok());

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
        weight,
        link_to,
        nav_label,
    })
}

/// Discover navigation items from the content directory structure.
///
/// Rules:
/// - Top-level `.md` files (except `_index.md`) become nav items (pages)
/// - Directories containing `_index.md` become nav items (sections)
/// - Items are sorted by weight (lower first), then alphabetically by label
pub fn discover_nav(content_dir: &Path) -> Result<Vec<NavItem>> {
    let mut nav_items = Vec::new();

    // Read top-level entries in content directory
    let entries = fs::read_dir(content_dir).map_err(|e| Error::ReadFile {
        path: content_dir.to_path_buf(),
        source: e,
    })?;

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            // Top-level .md file (except _index.md) → page nav item
            if path.extension().is_some_and(|ext| ext == "md") {
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if file_name != "_index.md" {
                    let content = Content::from_path(&path, ContentKind::Page)?;
                    let slug = path.file_stem().and_then(|s| s.to_str()).unwrap_or("page");
                    nav_items.push(NavItem {
                        label: content
                            .frontmatter
                            .nav_label
                            .unwrap_or(content.frontmatter.title),
                        path: format!("/{}.html", slug),
                        weight: content.frontmatter.weight.unwrap_or(50),
                    });
                }
            }
        } else if path.is_dir() {
            // Directory with _index.md → section nav item
            let index_path = path.join("_index.md");
            if index_path.exists() {
                let content = Content::from_path(&index_path, ContentKind::Section)?;
                let dir_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("section");
                nav_items.push(NavItem {
                    label: content
                        .frontmatter
                        .nav_label
                        .unwrap_or(content.frontmatter.title),
                    path: format!("/{}/index.html", dir_name),
                    weight: content.frontmatter.weight.unwrap_or(50),
                });
            }
        }
    }

    // Sort by weight, then alphabetically by label
    nav_items.sort_by(|a, b| a.weight.cmp(&b.weight).then_with(|| a.label.cmp(&b.label)));

    Ok(nav_items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("failed to create temp dir")
    }

    fn write_frontmatter(path: &Path, title: &str, weight: Option<i64>, nav_label: Option<&str>) {
        let mut content = format!("---\ntitle: \"{}\"\n", title);
        if let Some(w) = weight {
            content.push_str(&format!("weight: {}\n", w));
        }
        if let Some(label) = nav_label {
            content.push_str(&format!("nav_label: \"{}\"\n", label));
        }
        content.push_str("---\n\nBody content.");
        fs::write(path, content).expect("failed to write test file");
    }

    #[test]
    fn test_discover_nav_finds_pages() {
        let dir = create_test_dir();
        let content_dir = dir.path();

        // Create top-level page
        write_frontmatter(&content_dir.join("about.md"), "About Me", None, None);

        let nav = discover_nav(content_dir).expect("discover_nav failed");
        assert_eq!(nav.len(), 1);
        assert_eq!(nav[0].label, "About Me");
        assert_eq!(nav[0].path, "/about.html");
    }

    #[test]
    fn test_discover_nav_finds_sections() {
        let dir = create_test_dir();
        let content_dir = dir.path();

        // Create section directory with _index.md
        let blog_dir = content_dir.join("blog");
        fs::create_dir(&blog_dir).expect("failed to create blog dir");
        write_frontmatter(&blog_dir.join("_index.md"), "Blog", None, None);

        let nav = discover_nav(content_dir).expect("discover_nav failed");
        assert_eq!(nav.len(), 1);
        assert_eq!(nav[0].label, "Blog");
        assert_eq!(nav[0].path, "/blog/index.html");
    }

    #[test]
    fn test_discover_nav_excludes_root_index() {
        let dir = create_test_dir();
        let content_dir = dir.path();

        // Create _index.md at root (should be excluded from nav)
        write_frontmatter(&content_dir.join("_index.md"), "Home", None, None);
        write_frontmatter(&content_dir.join("about.md"), "About", None, None);

        let nav = discover_nav(content_dir).expect("discover_nav failed");
        assert_eq!(nav.len(), 1);
        assert_eq!(nav[0].label, "About");
    }

    #[test]
    fn test_discover_nav_sorts_by_weight() {
        let dir = create_test_dir();
        let content_dir = dir.path();

        write_frontmatter(&content_dir.join("about.md"), "About", Some(30), None);
        write_frontmatter(&content_dir.join("contact.md"), "Contact", Some(10), None);
        write_frontmatter(&content_dir.join("blog.md"), "Blog", Some(20), None);

        let nav = discover_nav(content_dir).expect("discover_nav failed");
        assert_eq!(nav.len(), 3);
        assert_eq!(nav[0].label, "Contact"); // weight 10
        assert_eq!(nav[1].label, "Blog"); // weight 20
        assert_eq!(nav[2].label, "About"); // weight 30
    }

    #[test]
    fn test_discover_nav_uses_nav_label() {
        let dir = create_test_dir();
        let content_dir = dir.path();

        write_frontmatter(
            &content_dir.join("about.md"),
            "About The Author",
            None,
            Some("About"),
        );

        let nav = discover_nav(content_dir).expect("discover_nav failed");
        assert_eq!(nav.len(), 1);
        assert_eq!(nav[0].label, "About"); // Uses nav_label, not title
    }
}
