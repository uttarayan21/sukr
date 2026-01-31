//! HTML templates using maud.

use crate::config::SiteConfig;
use crate::content::{Content, Frontmatter, NavItem};
use maud::{html, Markup, PreEscaped, DOCTYPE};

/// Compute relative path prefix based on page depth.
/// depth=0 (root) → "."
/// depth=1 (one level deep) → ".."
/// depth=2 → "../.."
fn relative_prefix(depth: usize) -> String {
    if depth == 0 {
        ".".to_string()
    } else {
        (0..depth).map(|_| "..").collect::<Vec<_>>().join("/")
    }
}

/// Calculate directory depth from page path.
/// "/index.html" → 0
/// "/about.html" → 0  
/// "/blog/index.html" → 1
/// "/blog/slug.html" → 1
fn path_depth(page_path: &str) -> usize {
    // Count segments: split by '/', filter empties, subtract 1 for filename
    let segments: Vec<_> = page_path.split('/').filter(|s| !s.is_empty()).collect();
    segments.len().saturating_sub(1)
}

/// Render a blog post with the base layout.
pub fn render_post(
    frontmatter: &Frontmatter,
    content_html: &str,
    page_path: &str,
    config: &SiteConfig,
    nav: &[NavItem],
) -> Markup {
    let depth = path_depth(page_path);
    let prefix = relative_prefix(depth);
    base_layout(
        &frontmatter.title,
        html! {
            article.post {
                header {
                    h1 { (frontmatter.title) }
                    @if let Some(ref date) = frontmatter.date {
                        time.date { (date) }
                    }
                    @if let Some(ref desc) = frontmatter.description {
                        p.description { (desc) }
                    }
                    @if !frontmatter.tags.is_empty() {
                        ul.tags {
                            @for tag in &frontmatter.tags {
                                li { a href=(format!("{}/tags/{}.html", prefix, tag)) { (tag) } }
                            }
                        }
                    }
                }
                section.content {
                    (PreEscaped(content_html))
                }
            }
        },
        page_path,
        config,
        nav,
    )
}

/// Render a standalone page (about, collab, etc.)
pub fn render_page(
    frontmatter: &Frontmatter,
    content_html: &str,
    page_path: &str,
    config: &SiteConfig,
    nav: &[NavItem],
) -> Markup {
    base_layout(
        &frontmatter.title,
        html! {
            article.page {
                h1 { (frontmatter.title) }
                section.content {
                    (PreEscaped(content_html))
                }
            }
        },
        page_path,
        config,
        nav,
    )
}

/// Render the homepage.
pub fn render_homepage(
    frontmatter: &Frontmatter,
    content_html: &str,
    page_path: &str,
    config: &SiteConfig,
    nav: &[NavItem],
) -> Markup {
    base_layout(
        &frontmatter.title,
        html! {
            section.hero {
                h1 { (frontmatter.title) }
                @if let Some(ref desc) = frontmatter.description {
                    p.tagline { (desc) }
                }
            }
            section.content {
                (PreEscaped(content_html))
            }
        },
        page_path,
        config,
        nav,
    )
}

/// Render the blog listing page.
pub fn render_blog_index(
    title: &str,
    posts: &[Content],
    page_path: &str,
    config: &SiteConfig,
    nav: &[NavItem],
) -> Markup {
    base_layout(
        title,
        html! {
            h1 { (title) }
            ul.post-list {
                @for post in posts {
                    li {
                        // Posts are .html files in the same directory
                        a href=(format!("./{}.html", post.slug)) {
                            span.title { (post.frontmatter.title) }
                            @if let Some(ref date) = post.frontmatter.date {
                                time.date { (date) }
                            }
                        }
                        @if let Some(ref desc) = post.frontmatter.description {
                            p.description { (desc) }
                        }
                    }
                }
            }
        },
        page_path,
        config,
        nav,
    )
}

/// Render the projects page with cards.
pub fn render_projects_index(
    title: &str,
    projects: &[Content],
    page_path: &str,
    config: &SiteConfig,
    nav: &[NavItem],
) -> Markup {
    base_layout(
        title,
        html! {
            h1 { (title) }
            ul.project-cards {
                @for project in projects {
                    li.card {
                        @if let Some(ref link) = project.frontmatter.link_to {
                            a href=(link) target="_blank" rel="noopener" {
                                h2 { (project.frontmatter.title) }
                                @if let Some(ref desc) = project.frontmatter.description {
                                    p { (desc) }
                                }
                            }
                        } @else {
                            h2 { (project.frontmatter.title) }
                            @if let Some(ref desc) = project.frontmatter.description {
                                p { (desc) }
                            }
                        }
                    }
                }
            }
        },
        page_path,
        config,
        nav,
    )
}

/// Base HTML layout wrapper.
fn base_layout(
    title: &str,
    content: Markup,
    page_path: &str,
    config: &SiteConfig,
    nav: &[NavItem],
) -> Markup {
    let depth = path_depth(page_path);
    let prefix = relative_prefix(depth);
    let canonical_url = format!("{}{}", config.base_url.trim_end_matches('/'), page_path);

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " | " (config.title) }
                link rel="canonical" href=(canonical_url);
                link rel="alternate" type="application/atom+xml" title="Atom Feed" href=(format!("{}/feed.xml", config.base_url.trim_end_matches('/')));
                link rel="stylesheet" href=(format!("{}/style.css", prefix));
            }
            body {
                nav {
                    a href=(format!("{}/index.html", prefix)) { (config.title) }
                    @for item in nav {
                        a href=(format!("{}{}", prefix, item.path)) { (item.label) }
                    }
                }
                main {
                    (content)
                }
                footer {
                    p { "© " (config.author) }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_prefix() {
        assert_eq!(relative_prefix(0), ".");
        assert_eq!(relative_prefix(1), "..");
        assert_eq!(relative_prefix(2), "../..");
        assert_eq!(relative_prefix(3), "../../..");
    }
}
