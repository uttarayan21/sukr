//! HTML templates using maud.

use crate::content::{Content, Frontmatter};
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

/// Render a blog post with the base layout.
pub fn render_post(frontmatter: &Frontmatter, content_html: &str, depth: usize) -> Markup {
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
        depth,
    )
}

/// Render a standalone page (about, collab, etc.)
pub fn render_page(frontmatter: &Frontmatter, content_html: &str, depth: usize) -> Markup {
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
        depth,
    )
}

/// Render the homepage.
pub fn render_homepage(frontmatter: &Frontmatter, content_html: &str, depth: usize) -> Markup {
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
        depth,
    )
}

/// Render the blog listing page.
pub fn render_blog_index(title: &str, posts: &[Content], depth: usize) -> Markup {
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
        depth,
    )
}

/// Render the projects page with cards.
pub fn render_projects_index(title: &str, projects: &[Content], depth: usize) -> Markup {
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
        depth,
    )
}

/// Base HTML layout wrapper.
fn base_layout(title: &str, content: Markup, depth: usize) -> Markup {
    let prefix = relative_prefix(depth);
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " | nrd.sh" }
                link rel="stylesheet" href=(format!("{}/style.css", prefix));
            }
            body {
                nav {
                    a href=(format!("{}/index.html", prefix)) { "nrd.sh" }
                    a href=(format!("{}/blog/index.html", prefix)) { "blog" }
                    a href=(format!("{}/projects/index.html", prefix)) { "projects" }
                    a href=(format!("{}/about.html", prefix)) { "about" }
                }
                main {
                    (content)
                }
                footer {
                    p { "© nrdxp" }
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
