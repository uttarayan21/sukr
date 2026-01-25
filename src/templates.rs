//! HTML templates using maud.

use crate::content::Frontmatter;
use maud::{html, Markup, DOCTYPE};

/// Render a blog post with the base layout.
pub fn render_post(frontmatter: &Frontmatter, content_html: &str) -> Markup {
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
                                li { a href=(format!("/tags/{}/", tag)) { (tag) } }
                            }
                        }
                    }
                }
                section.content {
                    (maud::PreEscaped(content_html))
                }
            }
        },
    )
}

/// Base HTML layout wrapper.
fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " | nrd.sh" }
                link rel="stylesheet" href="/style.css";
            }
            body {
                nav {
                    a href="/" { "nrd.sh" }
                    a href="/blog/" { "blog" }
                    a href="/projects/" { "projects" }
                    a href="/about/" { "about" }
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
