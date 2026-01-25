//! HTML templates using maud.

use crate::content::{Content, Frontmatter};
use maud::{DOCTYPE, Markup, PreEscaped, html};

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
                    (PreEscaped(content_html))
                }
            }
        },
    )
}

/// Render a standalone page (about, collab, etc.)
pub fn render_page(frontmatter: &Frontmatter, content_html: &str) -> Markup {
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
    )
}

/// Render the homepage.
pub fn render_homepage(frontmatter: &Frontmatter, content_html: &str) -> Markup {
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
    )
}

/// Render the blog listing page.
pub fn render_blog_index(title: &str, posts: &[Content]) -> Markup {
    base_layout(
        title,
        html! {
            h1 { (title) }
            ul.post-list {
                @for post in posts {
                    li {
                        a href=(format!("/blog/{}/", post.slug)) {
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
    )
}

/// Render the projects page with cards.
pub fn render_projects_index(title: &str, projects: &[Content]) -> Markup {
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
