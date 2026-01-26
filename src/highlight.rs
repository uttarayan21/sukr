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
    Bash,
    C,
    Css,
    Go,
    Html,
    JavaScript,
    Json,
    Nix,
    Python,
    Rust,
    TypeScript,
    Yaml,
}

impl Language {
    /// Parse a language identifier from a code fence.
    pub fn from_fence(lang: &str) -> Option<Self> {
        match lang.to_lowercase().as_str() {
            "bash" | "sh" | "shell" | "zsh" => Some(Language::Bash),
            "c" => Some(Language::C),
            "css" => Some(Language::Css),
            "go" | "golang" => Some(Language::Go),
            "html" => Some(Language::Html),
            "javascript" | "js" => Some(Language::JavaScript),
            "json" => Some(Language::Json),
            "nix" => Some(Language::Nix),
            "python" | "py" => Some(Language::Python),
            "rust" | "rs" => Some(Language::Rust),
            "typescript" | "ts" | "tsx" => Some(Language::TypeScript),
            "yaml" | "yml" => Some(Language::Yaml),
            _ => None,
        }
    }
}

/// Get highlight configuration for a language.
fn get_config(lang: Language) -> HighlightConfiguration {
    let (language, name, highlights) = match lang {
        Language::Bash => (
            tree_sitter_bash::LANGUAGE.into(),
            "bash",
            tree_sitter_bash::HIGHLIGHT_QUERY,
        ),
        Language::C => (
            tree_sitter_c::LANGUAGE.into(),
            "c",
            tree_sitter_c::HIGHLIGHT_QUERY,
        ),
        Language::Css => (
            tree_sitter_css::LANGUAGE.into(),
            "css",
            tree_sitter_css::HIGHLIGHTS_QUERY,
        ),
        Language::Go => (
            tree_sitter_go::LANGUAGE.into(),
            "go",
            tree_sitter_go::HIGHLIGHTS_QUERY,
        ),
        Language::Html => (
            tree_sitter_html::LANGUAGE.into(),
            "html",
            tree_sitter_html::HIGHLIGHTS_QUERY,
        ),
        Language::JavaScript => (
            tree_sitter_javascript::LANGUAGE.into(),
            "javascript",
            tree_sitter_javascript::HIGHLIGHT_QUERY,
        ),
        Language::Json => (
            tree_sitter_json::LANGUAGE.into(),
            "json",
            tree_sitter_json::HIGHLIGHTS_QUERY,
        ),
        Language::Nix => (
            tree_sitter_nix::LANGUAGE.into(),
            "nix",
            tree_sitter_nix::HIGHLIGHTS_QUERY,
        ),
        Language::Python => (
            tree_sitter_python::LANGUAGE.into(),
            "python",
            tree_sitter_python::HIGHLIGHTS_QUERY,
        ),
        Language::Rust => (
            tree_sitter_rust::LANGUAGE.into(),
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
        ),
        Language::TypeScript => (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            "typescript",
            tree_sitter_typescript::HIGHLIGHTS_QUERY,
        ),
        Language::Yaml => (
            tree_sitter_yaml::LANGUAGE.into(),
            "yaml",
            tree_sitter_yaml::HIGHLIGHTS_QUERY,
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
    let result = renderer.render(highlights, source.as_bytes(), &|highlight, buf| {
        let attrs = HTML_ATTRS.get(highlight.0).copied().unwrap_or(b"");
        buf.extend_from_slice(attrs);
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
        assert_eq!(Language::from_fence("nix"), Some(Language::Nix));
        assert_eq!(Language::from_fence("python"), Some(Language::Python));
        assert_eq!(Language::from_fence("py"), Some(Language::Python));
        assert_eq!(
            Language::from_fence("javascript"),
            Some(Language::JavaScript)
        );
        assert_eq!(Language::from_fence("js"), Some(Language::JavaScript));
        assert_eq!(
            Language::from_fence("typescript"),
            Some(Language::TypeScript)
        );
        assert_eq!(Language::from_fence("ts"), Some(Language::TypeScript));
        assert_eq!(Language::from_fence("tsx"), Some(Language::TypeScript));
        assert_eq!(Language::from_fence("go"), Some(Language::Go));
        assert_eq!(Language::from_fence("golang"), Some(Language::Go));
        assert_eq!(Language::from_fence("c"), Some(Language::C));
        assert_eq!(Language::from_fence("yaml"), Some(Language::Yaml));
        assert_eq!(Language::from_fence("yml"), Some(Language::Yaml));
        assert_eq!(Language::from_fence("css"), Some(Language::Css));
        assert_eq!(Language::from_fence("html"), Some(Language::Html));
        assert_eq!(Language::from_fence("unknown"), None);
    }

    #[test]
    fn test_highlight_rust_code() {
        let code = "fn main() { println!(\"hello\"); }";
        let html = highlight_code(Language::Rust, code);

        assert!(html.contains("class=\"hl-"));
        assert!(html.contains("fn"));
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
    fn test_highlight_nix_code() {
        let code = "{ pkgs, ... }: { environment.systemPackages = [ pkgs.vim ]; }";
        let html = highlight_code(Language::Nix, code);

        assert!(html.contains("class=\"hl-"));
        assert!(html.contains("pkgs"));
    }

    #[test]
    fn test_highlight_python_code() {
        let code = "def hello():\n    print(\"world\")";
        let html = highlight_code(Language::Python, code);

        assert!(html.contains("class=\"hl-"));
        assert!(html.contains("def"));
    }

    #[test]
    fn test_html_escape_fallback() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(!escaped.contains('<'));
        assert!(escaped.contains("&lt;"));
    }
}
