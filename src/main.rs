//! nrd.sh - Bespoke static site compiler.
//!
//! Transforms markdown content into a minimal static site.

mod content;
mod error;
mod render;
mod templates;

use crate::content::Content;
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

    if !content_dir.exists() {
        return Err(Error::ContentDirNotFound(content_dir.to_path_buf()));
    }

    // For MVP: process all markdown files in content/blog/
    let blog_dir = content_dir.join("blog");

    for entry in walkdir::WalkDir::new(&blog_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().map_or(false, |ext| ext == "md")
                && e.path().file_name().map_or(false, |n| n != "_index.md")
        })
    {
        let path = entry.path();
        eprintln!("processing: {}", path.display());

        let content = Content::from_path(path)?;
        let html_body = render::markdown_to_html(&content.body);
        let page = templates::render_post(&content.frontmatter, &html_body);

        let out_path = output_dir.join(content.output_path(content_dir));
        let out_dir = out_path.parent().unwrap();

        fs::create_dir_all(out_dir).map_err(|e| Error::CreateDir {
            path: out_dir.to_path_buf(),
            source: e,
        })?;

        fs::write(&out_path, page.into_string()).map_err(|e| Error::WriteFile {
            path: out_path.clone(),
            source: e,
        })?;

        eprintln!("  → {}", out_path.display());
    }

    eprintln!("done!");
    Ok(())
}
