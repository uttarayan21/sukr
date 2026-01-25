//! Syntax highlighting via tree-sitter.

use tree_sitter_highlight::{HighlightConfiguration, Highlighter as TSHighlighter, HtmlRenderer};

/// Recognized highlight names mapped to CSS classes.
/// Order matters: index becomes the class name suffix.
const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "function",
    "function.builtin",
    "keyword",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "string",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

/// Static HTML attributes for each highlight class.
/// Pre-computed to avoid allocations in the render loop.
/// HtmlRenderer wraps with <span ...>...</span>, callback returns just the attributes.
const HTML_ATTRS: &[&[u8]] = &[
    b" class=\"hl-attribute\"",
    b" class=\"hl-comment\"",
    b" class=\"hl-constant\"",
    b" class=\"hl-constant-builtin\"",
    b" class=\"hl-constructor\"",
    b" class=\"hl-function\"",
    b" class=\"hl-function-builtin\"",
    b" class=\"hl-keyword\"",
    b" class=\"hl-number\"",
    b" class=\"hl-operator\"",
    b" class=\"hl-property\"",
    b" class=\"hl-punctuation\"",
    b" class=\"hl-punctuation-bracket\"",
    b" class=\"hl-punctuation-delimiter\"",
    b" class=\"hl-string\"",
    b" class=\"hl-type\"",
    b" class=\"hl-type-builtin\"",
    b" class=\"hl-variable\"",
    b" class=\"hl-variable-builtin\"",
    b" class=\"hl-variable-parameter\"",
];

/// Supported languages for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
    Bash,
    Json,
}

impl Language {
    /// Parse a language identifier from a code fence.
    pub fn from_fence(lang: &str) -> Option<Self> {
        match lang.to_lowercase().as_str() {
            "rust" | "rs" => Some(Language::Rust),
            "bash" | "sh" | "shell" | "zsh" => Some(Language::Bash),
            "json" => Some(Language::Json),
            _ => None,
        }
    }
}

/// Get highlight configuration for a language.
fn get_config(lang: Language) -> HighlightConfiguration {
    let (language, name, highlights) = match lang {
        Language::Rust => (
            tree_sitter_rust::LANGUAGE.into(),
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
        ),
        Language::Bash => (
            tree_sitter_bash::LANGUAGE.into(),
            "bash",
            tree_sitter_bash::HIGHLIGHT_QUERY,
        ),
        Language::Json => (
            tree_sitter_json::LANGUAGE.into(),
            "json",
            tree_sitter_json::HIGHLIGHTS_QUERY,
        ),
    };

    let mut config = HighlightConfiguration::new(language, name, highlights, "", "")
        .expect("highlight query should be valid");

    config.configure(HIGHLIGHT_NAMES);
    config
}

/// Highlight source code and return HTML with span elements.
pub fn highlight_code(lang: Language, source: &str) -> String {
    let mut highlighter = TSHighlighter::new();
    let config = get_config(lang);

    let highlights = match highlighter.highlight(&config, source.as_bytes(), None, |_| None) {
        Ok(h) => h,
        Err(_) => return html_escape(source),
    };

    let mut renderer = HtmlRenderer::new();
    let result = renderer.render(highlights, source.as_bytes(), &|highlight| {
        HTML_ATTRS.get(highlight.0).copied().unwrap_or(b"<span>")
    });

    match result {
        Ok(()) => String::from_utf8_lossy(&renderer.html).into_owned(),
        Err(_) => html_escape(source),
    }
}

/// Simple HTML escape for fallback.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_fence() {
        assert_eq!(Language::from_fence("rust"), Some(Language::Rust));
        assert_eq!(Language::from_fence("rs"), Some(Language::Rust));
        assert_eq!(Language::from_fence("bash"), Some(Language::Bash));
        assert_eq!(Language::from_fence("sh"), Some(Language::Bash));
        assert_eq!(Language::from_fence("json"), Some(Language::Json));
        assert_eq!(Language::from_fence("unknown"), None);
    }

    #[test]
    fn test_highlight_rust_code() {
        let code = "fn main() { println!(\"hello\"); }";
        let html = highlight_code(Language::Rust, code);

        // Should contain span elements with highlight classes
        assert!(html.contains("class=\"hl-"));
        // Should contain the keyword "fn"
        assert!(html.contains("fn"));
        // Should contain the string
        assert!(html.contains("hello"));
    }

    #[test]
    fn test_highlight_bash_code() {
        let code = "#!/bin/bash\necho \"hello world\"";
        let html = highlight_code(Language::Bash, code);

        assert!(html.contains("class=\"hl-"));
        assert!(html.contains("echo"));
    }

    #[test]
    fn test_html_escape_fallback() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains('<'));
        assert!(escaped.contains("&lt;"));
    }
}
