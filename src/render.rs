//! Markdown to HTML rendering via pulldown-cmark.

use pulldown_cmark::{html, Options, Parser};

/// Render markdown content to HTML.
///
/// Currently performs basic rendering. Future phases will intercept
/// code blocks for Tree-sitter highlighting.
pub fn markdown_to_html(markdown: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
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
}
