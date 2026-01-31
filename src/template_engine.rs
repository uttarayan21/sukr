//! Tera-based template engine for runtime HTML generation.

use std::path::Path;

use serde::Serialize;
use tera::{Context, Tera};

use crate::config::SiteConfig;
use crate::content::{Content, NavItem};
use crate::error::{Error, Result};

/// Runtime template engine wrapping Tera.
pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    /// Load templates from a directory (glob pattern: `templates/**/*`).
    pub fn new(template_dir: &Path) -> Result<Self> {
        let pattern = template_dir.join("**/*").display().to_string();
        let tera = Tera::new(&pattern).map_err(|e| Error::TemplateLoad {
            message: e.to_string(),
        })?;
        Ok(Self { tera })
    }

    /// Render a template by name with the given context.
    pub fn render(&self, template_name: &str, context: &Context) -> Result<String> {
        self.tera
            .render(template_name, context)
            .map_err(|e| Error::TemplateRender {
                template: template_name.to_string(),
                message: e.to_string(),
            })
    }

    /// Render a standalone page (about, collab, etc.).
    pub fn render_page(
        &self,
        content: &Content,
        html_body: &str,
        page_path: &str,
        config: &SiteConfig,
        nav: &[NavItem],
    ) -> Result<String> {
        let mut ctx = self.base_context(page_path, config, nav);
        ctx.insert("page", &FrontmatterContext::from(&content.frontmatter));
        ctx.insert("content", html_body);
        self.render("page.html", &ctx)
    }

    /// Render a content item (blog post, project, etc.).
    pub fn render_content(
        &self,
        content: &Content,
        html_body: &str,
        page_path: &str,
        config: &SiteConfig,
        nav: &[NavItem],
    ) -> Result<String> {
        let template = content
            .frontmatter
            .template
            .as_deref()
            .unwrap_or("content/default.html");
        let mut ctx = self.base_context(page_path, config, nav);
        ctx.insert("page", &FrontmatterContext::from(&content.frontmatter));
        ctx.insert("content", html_body);
        self.render(template, &ctx)
    }

    /// Render a section index page (blog index, projects index).
    pub fn render_section(
        &self,
        section: &Content,
        items: &[ContentContext],
        page_path: &str,
        config: &SiteConfig,
        nav: &[NavItem],
    ) -> Result<String> {
        let section_type = section
            .frontmatter
            .section_type
            .as_deref()
            .unwrap_or("default");
        let template = format!("section/{}.html", section_type);

        let mut ctx = self.base_context(page_path, config, nav);
        ctx.insert("section", &FrontmatterContext::from(&section.frontmatter));
        ctx.insert("items", items);
        self.render(&template, &ctx)
    }

    /// Build base context with common variables.
    fn base_context(&self, page_path: &str, config: &SiteConfig, nav: &[NavItem]) -> Context {
        let mut ctx = Context::new();
        ctx.insert("config", &ConfigContext::from(config));
        ctx.insert("nav", nav);
        ctx.insert("page_path", page_path);
        ctx.insert("prefix", &relative_prefix(page_path));
        ctx
    }
}

/// Compute relative path prefix based on page depth.
fn relative_prefix(page_path: &str) -> String {
    let depth = page_path.matches('/').count().saturating_sub(1);
    if depth == 0 {
        ".".to_string()
    } else {
        (0..depth).map(|_| "..").collect::<Vec<_>>().join("/")
    }
}

// ============================================================================
// Context structs for Tera serialization
// ============================================================================

/// Site config context for templates.
#[derive(Serialize)]
pub struct ConfigContext {
    pub title: String,
    pub author: String,
    pub base_url: String,
}

impl From<&SiteConfig> for ConfigContext {
    fn from(config: &SiteConfig) -> Self {
        Self {
            title: config.title.clone(),
            author: config.author.clone(),
            base_url: config.base_url.clone(),
        }
    }
}

/// Frontmatter context for templates.
#[derive(Serialize)]
pub struct FrontmatterContext {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub weight: Option<i64>,
    pub link_to: Option<String>,
}

impl From<&crate::content::Frontmatter> for FrontmatterContext {
    fn from(fm: &crate::content::Frontmatter) -> Self {
        Self {
            title: fm.title.clone(),
            description: fm.description.clone(),
            date: fm.date.clone(),
            tags: fm.tags.clone(),
            weight: fm.weight,
            link_to: fm.link_to.clone(),
        }
    }
}

/// Content item context for section listings.
#[derive(Serialize)]
pub struct ContentContext {
    pub frontmatter: FrontmatterContext,
    pub body: String,
    pub slug: String,
    pub path: String,
}

impl ContentContext {
    pub fn from_content(content: &Content, content_dir: &Path) -> Self {
        Self {
            frontmatter: FrontmatterContext::from(&content.frontmatter),
            body: content.body.clone(),
            slug: content.slug.clone(),
            path: format!("/{}", content.output_path(content_dir).display()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_prefix_root() {
        assert_eq!(relative_prefix("/index.html"), ".");
    }

    #[test]
    fn test_relative_prefix_depth_1() {
        assert_eq!(relative_prefix("/blog/index.html"), "..");
    }

    #[test]
    fn test_relative_prefix_depth_2() {
        assert_eq!(relative_prefix("/blog/posts/foo.html"), "../..");
    }
}
