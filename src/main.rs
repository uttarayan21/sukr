//! nrd.sh - Bespoke static site compiler.
//!
//! Transforms markdown content into a minimal static site.

mod config;
mod content;
mod css;
mod error;
mod highlight;
mod render;
mod templates;

use crate::content::{Content, ContentKind};
use crate::error::{Error, Result};
use std::fs;
use std::path::Path;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let content_dir = Path::new("content");
    let output_dir = Path::new("public");
    let static_dir = Path::new("static");
    let config_path = Path::new("site.toml");

    if !content_dir.exists() {
        return Err(Error::ContentDirNotFound(content_dir.to_path_buf()));
    }

    // Load site configuration
    let config = config::SiteConfig::load(config_path)?;

    // 0. Copy static assets
    copy_static_assets(static_dir, output_dir)?;

    // 1. Process blog posts
    let mut posts = process_blog_posts(content_dir, output_dir, &config)?;

    // 2. Generate blog index (sorted by date, newest first)
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
    generate_blog_index(output_dir, &posts, &config)?;

    // 3. Process standalone pages (about, collab)
    process_pages(content_dir, output_dir, &config)?;

    // 4. Process projects and generate project index
    let mut projects = process_projects(content_dir)?;
    projects.sort_by(|a, b| {
        a.frontmatter
            .weight
            .unwrap_or(99)
            .cmp(&b.frontmatter.weight.unwrap_or(99))
    });
    generate_projects_index(output_dir, &projects, &config)?;

    // 5. Generate homepage
    generate_homepage(content_dir, output_dir, &config)?;

    eprintln!("done!");
    Ok(())
}

/// Process all blog posts in content/blog/
fn process_blog_posts(
    content_dir: &Path,
    output_dir: &Path,
    config: &config::SiteConfig,
) -> Result<Vec<Content>> {
    let blog_dir = content_dir.join("blog");
    let mut posts = Vec::new();

    for entry in walkdir::WalkDir::new(&blog_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().is_some_and(|ext| ext == "md")
                && e.path().file_name().is_some_and(|n| n != "_index.md")
        })
    {
        let path = entry.path();
        eprintln!("processing: {}", path.display());

        let content = Content::from_path(path, ContentKind::Post)?;
        let html_body = render::markdown_to_html(&content.body);
        let page_path = format!("/{}", content.output_path(content_dir).display());
        let page = templates::render_post(&content.frontmatter, &html_body, &page_path, config);

        write_output(output_dir, content_dir, &content, page.into_string())?;
        posts.push(content);
    }

    Ok(posts)
}

/// Generate the blog listing page
fn generate_blog_index(
    output_dir: &Path,
    posts: &[Content],
    config: &config::SiteConfig,
) -> Result<()> {
    let out_path = output_dir.join("blog/index.html");
    eprintln!("generating: {}", out_path.display());

    let page = templates::render_blog_index("Blog", posts, "/blog/index.html", config);

    fs::create_dir_all(out_path.parent().unwrap()).map_err(|e| Error::CreateDir {
        path: out_path.parent().unwrap().to_path_buf(),
        source: e,
    })?;

    fs::write(&out_path, page.into_string()).map_err(|e| Error::WriteFile {
        path: out_path.clone(),
        source: e,
    })?;

    eprintln!("  → {}", out_path.display());
    Ok(())
}

/// Process standalone pages in content/ (about.md, collab.md)
fn process_pages(content_dir: &Path, output_dir: &Path, config: &config::SiteConfig) -> Result<()> {
    for name in ["about.md", "collab.md"] {
        let path = content_dir.join(name);
        if path.exists() {
            eprintln!("processing: {}", path.display());

            let content = Content::from_path(&path, ContentKind::Page)?;
            let html_body = render::markdown_to_html(&content.body);
            let page_path = format!("/{}", content.output_path(content_dir).display());
            let page = templates::render_page(&content.frontmatter, &html_body, &page_path, config);

            write_output(output_dir, content_dir, &content, page.into_string())?;
        }
    }
    Ok(())
}

/// Load all project cards (without writing individual pages)
fn process_projects(content_dir: &Path) -> Result<Vec<Content>> {
    let projects_dir = content_dir.join("projects");
    let mut projects = Vec::new();

    for entry in walkdir::WalkDir::new(&projects_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().is_some_and(|ext| ext == "md")
                && e.path().file_name().is_some_and(|n| n != "_index.md")
        })
    {
        let content = Content::from_path(entry.path(), ContentKind::Project)?;
        projects.push(content);
    }

    Ok(projects)
}

/// Generate the projects listing page
fn generate_projects_index(
    output_dir: &Path,
    projects: &[Content],
    config: &config::SiteConfig,
) -> Result<()> {
    let out_path = output_dir.join("projects/index.html");
    eprintln!("generating: {}", out_path.display());

    let page =
        templates::render_projects_index("Projects", projects, "/projects/index.html", config);

    fs::create_dir_all(out_path.parent().unwrap()).map_err(|e| Error::CreateDir {
        path: out_path.parent().unwrap().to_path_buf(),
        source: e,
    })?;

    fs::write(&out_path, page.into_string()).map_err(|e| Error::WriteFile {
        path: out_path.clone(),
        source: e,
    })?;

    eprintln!("  → {}", out_path.display());
    Ok(())
}

/// Generate the homepage from content/_index.md
fn generate_homepage(
    content_dir: &Path,
    output_dir: &Path,
    config: &config::SiteConfig,
) -> Result<()> {
    let index_path = content_dir.join("_index.md");
    eprintln!("generating: homepage");

    let content = Content::from_path(&index_path, ContentKind::Section)?;
    let html_body = render::markdown_to_html(&content.body);
    let page = templates::render_homepage(&content.frontmatter, &html_body, "/index.html", config);

    let out_path = output_dir.join("index.html");

    fs::create_dir_all(output_dir).map_err(|e| Error::CreateDir {
        path: output_dir.to_path_buf(),
        source: e,
    })?;

    fs::write(&out_path, page.into_string()).map_err(|e| Error::WriteFile {
        path: out_path.clone(),
        source: e,
    })?;

    eprintln!("  → {}", out_path.display());
    Ok(())
}

/// Write a content item to its output path
fn write_output(
    output_dir: &Path,
    content_dir: &Path,
    content: &Content,
    html: String,
) -> Result<()> {
    let out_path = output_dir.join(content.output_path(content_dir));
    let out_dir = out_path.parent().unwrap();

    fs::create_dir_all(out_dir).map_err(|e| Error::CreateDir {
        path: out_dir.to_path_buf(),
        source: e,
    })?;

    fs::write(&out_path, html).map_err(|e| Error::WriteFile {
        path: out_path.clone(),
        source: e,
    })?;

    eprintln!("  → {}", out_path.display());
    Ok(())
}

/// Copy static assets (CSS, images, etc.) to output directory.
/// CSS files are minified before writing.
fn copy_static_assets(static_dir: &Path, output_dir: &Path) -> Result<()> {
    use crate::css::minify_css;

    if !static_dir.exists() {
        return Ok(()); // No static dir is fine
    }

    fs::create_dir_all(output_dir).map_err(|e| Error::CreateDir {
        path: output_dir.to_path_buf(),
        source: e,
    })?;

    for entry in walkdir::WalkDir::new(static_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let src = entry.path();
        let relative = src.strip_prefix(static_dir).unwrap();
        let dest = output_dir.join(relative);

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).map_err(|e| Error::CreateDir {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        // Minify CSS files, copy others directly
        if src.extension().is_some_and(|ext| ext == "css") {
            let css = fs::read_to_string(src).map_err(|e| Error::ReadFile {
                path: src.to_path_buf(),
                source: e,
            })?;
            let minified = minify_css(&css);
            fs::write(&dest, &minified).map_err(|e| Error::WriteFile {
                path: dest.clone(),
                source: e,
            })?;
            eprintln!(
                "minifying: {} → {} ({} → {} bytes)",
                src.display(),
                dest.display(),
                css.len(),
                minified.len()
            );
        } else {
            fs::copy(src, &dest).map_err(|e| Error::WriteFile {
                path: dest.clone(),
                source: e,
            })?;
            eprintln!("copying: {} → {}", src.display(), dest.display());
        }
    }

    Ok(())
}
