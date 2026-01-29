//! Markdown to HTML rendering via pulldown-cmark with syntax highlighting.

use crate::highlight::{Language, highlight_code};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

/// Render markdown content to HTML with syntax highlighting.
pub fn markdown_to_html(markdown: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_MATH;

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    let mut code_block_lang: Option<String> = None;
    let mut code_block_content = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                // Extract language from code fence
                code_block_lang = match kind {
                    CodeBlockKind::Fenced(lang) => {
                        let lang_str = lang.as_ref().split_whitespace().next().unwrap_or("");
                        if lang_str.is_empty() {
                            None
                        } else {
                            Some(lang_str.to_string())
                        }
                    }
                    CodeBlockKind::Indented => None,
                };
                code_block_content.clear();
            }
            Event::Text(text) if code_block_lang.is_some() || !code_block_content.is_empty() => {
                // Accumulate code block content
                // Note: we're in a code block if we have a lang OR we've started accumulating
                if code_block_lang.is_some() {
                    code_block_content.push_str(&text);
                } else {
                    // Regular text, render normally
                    html_output.push_str(&html_escape(&text));
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                // Render the code block with highlighting
                let lang_str = code_block_lang.as_deref().unwrap_or("");
                html_output.push_str("<pre><code");

                if let Some(lang) = Language::from_fence(lang_str) {
                    // Supported language: apply tree-sitter highlighting
                    html_output.push_str(&format!(" class=\"language-{}\">", lang_str));
                    html_output.push_str(&highlight_code(lang, &code_block_content));
                } else {
                    // Unsupported language: render as plain escaped text
                    if !lang_str.is_empty() {
                        html_output.push_str(&format!(" class=\"language-{}\">", lang_str));
                    } else {
                        html_output.push('>');
                    }
                    html_output.push_str(&html_escape(&code_block_content));
                }

                html_output.push_str("</code></pre>\n");
                code_block_lang = None;
                code_block_content.clear();
            }
            Event::Text(text) => {
                // Regular text outside code blocks
                html_output.push_str(&html_escape(&text));
            }
            Event::Code(text) => {
                // Inline code
                html_output.push_str("<code>");
                html_output.push_str(&html_escape(&text));
                html_output.push_str("</code>");
            }
            Event::Start(tag) => {
                html_output.push_str(&start_tag_to_html(&tag));
            }
            Event::End(tag) => {
                html_output.push_str(&end_tag_to_html(&tag));
            }
            Event::SoftBreak => {
                html_output.push('\n');
            }
            Event::HardBreak => {
                html_output.push_str("<br />\n");
            }
            Event::Rule => {
                html_output.push_str("<hr />\n");
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                html_output.push_str(&html);
            }
            Event::FootnoteReference(name) => {
                html_output.push_str(&format!(
                    "<sup class=\"footnote-ref\"><a href=\"#fn-{}\">{}</a></sup>",
                    name, name
                ));
            }
            Event::TaskListMarker(checked) => {
                let checkbox = if checked {
                    "<input type=\"checkbox\" checked disabled />"
                } else {
                    "<input type=\"checkbox\" disabled />"
                };
                html_output.push_str(checkbox);
            }
            Event::InlineMath(latex) => match crate::math::render_math(&latex, false) {
                Ok(rendered) => html_output.push_str(&rendered),
                Err(e) => {
                    eprintln!("math render error: {e}");
                    html_output.push_str("<code class=\"math-error\">");
                    html_output.push_str(&html_escape(&latex));
                    html_output.push_str("</code>");
                }
            },
            Event::DisplayMath(latex) => match crate::math::render_math(&latex, true) {
                Ok(rendered) => {
                    html_output.push_str("<div class=\"math-display\">\n");
                    html_output.push_str(&rendered);
                    html_output.push_str("\n</div>\n");
                }
                Err(e) => {
                    eprintln!("math render error: {e}");
                    html_output.push_str("<pre class=\"math-error\">");
                    html_output.push_str(&html_escape(&latex));
                    html_output.push_str("</pre>\n");
                }
            },
        }
    }

    html_output
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn start_tag_to_html(tag: &Tag) -> String {
    match tag {
        Tag::Paragraph => "<p>".to_string(),
        Tag::Heading { level, .. } => format!("<h{}>", *level as u8),
        Tag::BlockQuote(_) => "<blockquote>\n".to_string(),
        Tag::CodeBlock(_) => String::new(), // Handled separately
        Tag::List(Some(start)) => format!("<ol start=\"{}\">\n", start),
        Tag::List(None) => "<ul>\n".to_string(),
        Tag::Item => "<li>".to_string(),
        Tag::FootnoteDefinition(name) => {
            format!("<div class=\"footnote\" id=\"fn-{}\">", name)
        }
        Tag::Table(_) => "<table>\n".to_string(),
        Tag::TableHead => "<thead>\n<tr>\n".to_string(),
        Tag::TableRow => "<tr>\n".to_string(),
        Tag::TableCell => "<td>".to_string(),
        Tag::Emphasis => "<em>".to_string(),
        Tag::Strong => "<strong>".to_string(),
        Tag::Strikethrough => "<del>".to_string(),
        Tag::Link {
            dest_url, title, ..
        } => {
            if title.is_empty() {
                format!("<a href=\"{}\">", dest_url)
            } else {
                format!("<a href=\"{}\" title=\"{}\">", dest_url, title)
            }
        }
        Tag::Image {
            dest_url, title, ..
        } => {
            format!("<img src=\"{}\" alt=\"\" title=\"{}\" />", dest_url, title)
        }
        Tag::HtmlBlock => String::new(),
        Tag::MetadataBlock(_) => String::new(),
        Tag::DefinitionListTitle => "<dt>".to_string(),
        Tag::DefinitionListDefinition => "<dd>".to_string(),
        Tag::DefinitionList => "<dl>".to_string(),
    }
}

fn end_tag_to_html(tag: &TagEnd) -> String {
    match tag {
        TagEnd::Paragraph => "</p>\n".to_string(),
        TagEnd::Heading(level) => format!("</h{}>\n", *level as u8),
        TagEnd::BlockQuote(_) => "</blockquote>\n".to_string(),
        TagEnd::CodeBlock => String::new(), // Handled separately
        TagEnd::List(ordered) => {
            if *ordered {
                "</ol>\n".to_string()
            } else {
                "</ul>\n".to_string()
            }
        }
        TagEnd::Item => "</li>\n".to_string(),
        TagEnd::FootnoteDefinition => "</div>\n".to_string(),
        TagEnd::Table => "</table>\n".to_string(),
        TagEnd::TableHead => "</tr>\n</thead>\n".to_string(),
        TagEnd::TableRow => "</tr>\n".to_string(),
        TagEnd::TableCell => "</td>\n".to_string(),
        TagEnd::Emphasis => "</em>".to_string(),
        TagEnd::Strong => "</strong>".to_string(),
        TagEnd::Strikethrough => "</del>".to_string(),
        TagEnd::Link => "</a>".to_string(),
        TagEnd::Image => String::new(),
        TagEnd::HtmlBlock => String::new(),
        TagEnd::MetadataBlock(_) => String::new(),
        TagEnd::DefinitionListTitle => "</dt>\n".to_string(),
        TagEnd::DefinitionListDefinition => "</dd>\n".to_string(),
        TagEnd::DefinitionList => "</dl>\n".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let md = "# Hello\n\nThis is a *test*.";
        let html = markdown_to_html(md);
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<em>test</em>"));
    }

    #[test]
    fn test_code_block_highlighting() {
        let md = "```rust\nfn main() {}\n```";
        let html = markdown_to_html(md);

        // Should contain highlighted code
        assert!(html.contains("<pre><code"));
        assert!(html.contains("language-rust"));
        assert!(html.contains("class=\"hl-"));
    }

    #[test]
    fn test_code_block_unknown_language() {
        let md = "```unknown\nsome code\n```";
        let html = markdown_to_html(md);

        // Should contain escaped code without highlighting spans
        assert!(html.contains("<pre><code"));
        assert!(html.contains("language-unknown"));
        assert!(html.contains("some code"));
        assert!(!html.contains("class=\"hl-"));
    }

    #[test]
    fn test_inline_code() {
        let md = "Use `cargo run` to start.";
        let html = markdown_to_html(md);

        assert!(html.contains("<code>cargo run</code>"));
    }
}
