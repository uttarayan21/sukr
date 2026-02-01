//! Syntax highlighting via tree-sitter.

use std::sync::LazyLock;
use tree_sitter_highlight::{HighlightConfiguration, Highlighter as TSHighlighter, HtmlRenderer};

/// Recognized highlight names mapped to CSS classes.
/// Order matters: index becomes the class name suffix.
/// Comprehensive list covering captures from all supported languages.
const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "escape",
    "function",
    "function.builtin",
    "keyword",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.escape",
    "string.special",
    "string.special.path",
    "string.special.uri",
    "text.literal",
    "text.reference",
    "text.title",
    "text.uri",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

/// Static HTML attributes for each highlight class.
/// Pre-computed to avoid allocations in the render loop.
/// Must be in same order as HIGHLIGHT_NAMES.
const HTML_ATTRS: &[&[u8]] = &[
    b" class=\"hl-attribute\"",
    b" class=\"hl-comment\"",
    b" class=\"hl-constant\"",
    b" class=\"hl-constant-builtin\"",
    b" class=\"hl-constructor\"",
    b" class=\"hl-embedded\"",
    b" class=\"hl-escape\"",
    b" class=\"hl-function\"",
    b" class=\"hl-function-builtin\"",
    b" class=\"hl-keyword\"",
    b" class=\"hl-number\"",
    b" class=\"hl-operator\"",
    b" class=\"hl-property\"",
    b" class=\"hl-punctuation\"",
    b" class=\"hl-punctuation-bracket\"",
    b" class=\"hl-punctuation-delimiter\"",
    b" class=\"hl-punctuation-special\"",
    b" class=\"hl-string\"",
    b" class=\"hl-string-escape\"",
    b" class=\"hl-string-special\"",
    b" class=\"hl-string-special-path\"",
    b" class=\"hl-string-special-uri\"",
    b" class=\"hl-text-literal\"",
    b" class=\"hl-text-reference\"",
    b" class=\"hl-text-title\"",
    b" class=\"hl-text-uri\"",
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
    Markdown,
    Nix,
    Python,
    Rust,
    Toml,
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
            "markdown" | "md" => Some(Language::Markdown),
            "nix" => Some(Language::Nix),
            "python" | "py" => Some(Language::Python),
            "rust" | "rs" => Some(Language::Rust),
            "toml" => Some(Language::Toml),
            "typescript" | "ts" | "tsx" => Some(Language::TypeScript),
            "yaml" | "yml" => Some(Language::Yaml),
            _ => None,
        }
    }
}

/// Helper to create and configure a HighlightConfiguration.
fn make_config(
    language: tree_sitter::Language,
    name: &str,
    highlights: &str,
    injections: &str,
) -> HighlightConfiguration {
    let mut config = HighlightConfiguration::new(language, name, highlights, injections, "")
        .expect("highlight query should be valid");
    config.configure(HIGHLIGHT_NAMES);
    config
}

// Static configurations for each language, lazily initialized.

static BASH_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_bash::LANGUAGE.into(),
        "bash",
        tree_sitter_bash::HIGHLIGHT_QUERY,
        "",
    )
});

static C_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_c::LANGUAGE.into(),
        "c",
        tree_sitter_c::HIGHLIGHT_QUERY,
        "",
    )
});

static CSS_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_css::LANGUAGE.into(),
        "css",
        tree_sitter_css::HIGHLIGHTS_QUERY,
        "",
    )
});

static GO_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_go::LANGUAGE.into(),
        "go",
        tree_sitter_go::HIGHLIGHTS_QUERY,
        "",
    )
});

static HTML_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_html::LANGUAGE.into(),
        "html",
        tree_sitter_html::HIGHLIGHTS_QUERY,
        tree_sitter_html::INJECTIONS_QUERY,
    )
});

static JAVASCRIPT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_javascript::LANGUAGE.into(),
        "javascript",
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::INJECTIONS_QUERY,
    )
});

static JSON_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_json::LANGUAGE.into(),
        "json",
        tree_sitter_json::HIGHLIGHTS_QUERY,
        "",
    )
});

static MARKDOWN_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_md::LANGUAGE.into(),
        "markdown",
        tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
        include_str!("../queries/md-injections.scm"),
    )
});

static NIX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_nix::LANGUAGE.into(),
        "nix",
        include_str!("../queries/nix-highlights.scm"),
        include_str!("../queries/nix-injections.scm"),
    )
});

static PYTHON_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_python::LANGUAGE.into(),
        "python",
        tree_sitter_python::HIGHLIGHTS_QUERY,
        "",
    )
});

static RUST_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_rust::LANGUAGE.into(),
        "rust",
        tree_sitter_rust::HIGHLIGHTS_QUERY,
        "",
    )
});

static TOML_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_toml_ng::LANGUAGE.into(),
        "toml",
        tree_sitter_toml_ng::HIGHLIGHTS_QUERY,
        "",
    )
});

static TYPESCRIPT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "typescript",
        tree_sitter_typescript::HIGHLIGHTS_QUERY,
        "",
    )
});

static YAML_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    make_config(
        tree_sitter_yaml::LANGUAGE.into(),
        "yaml",
        tree_sitter_yaml::HIGHLIGHTS_QUERY,
        "",
    )
});

/// Get a static reference to the highlight configuration for a language.
fn get_config(lang: Language) -> &'static HighlightConfiguration {
    match lang {
        Language::Bash => &BASH_CONFIG,
        Language::C => &C_CONFIG,
        Language::Css => &CSS_CONFIG,
        Language::Go => &GO_CONFIG,
        Language::Html => &HTML_CONFIG,
        Language::JavaScript => &JAVASCRIPT_CONFIG,
        Language::Json => &JSON_CONFIG,
        Language::Markdown => &MARKDOWN_CONFIG,
        Language::Nix => &NIX_CONFIG,
        Language::Python => &PYTHON_CONFIG,
        Language::Rust => &RUST_CONFIG,
        Language::Toml => &TOML_CONFIG,
        Language::TypeScript => &TYPESCRIPT_CONFIG,
        Language::Yaml => &YAML_CONFIG,
    }
}

/// Get config by language name string (for injection callback).
fn get_config_by_name(name: &str) -> Option<&'static HighlightConfiguration> {
    Language::from_fence(name).map(get_config)
}

/// Highlight source code and return HTML with span elements.
///
/// Uses tree-sitter-highlight with injection support for embedded languages
/// in Nix, HTML, and JavaScript code blocks.
pub fn highlight_code(lang: Language, source: &str) -> String {
    let config = get_config(lang);

    // Leak both the highlighter and source to satisfy 'static lifetime.
    // Acceptable for SSG where the process exits after building.
    let highlighter: &'static mut TSHighlighter = Box::leak(Box::new(TSHighlighter::new()));
    let static_source: &'static str = Box::leak(source.to_owned().into_boxed_str());
    let source_bytes: &'static [u8] = static_source.as_bytes();

    let highlights = match highlighter.highlight(config, source_bytes, None, get_config_by_name) {
        Ok(h) => h,
        Err(_) => return html_escape(source),
    };

    let mut renderer = HtmlRenderer::new();
    let result = renderer.render(highlights, source_bytes, &|highlight, buf| {
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

    #[test]
    fn test_nix_injection_bash_buildphase() {
        // Nix code with embedded bash in buildPhase
        let code = r#"{ pkgs }:
pkgs.stdenv.mkDerivation {
  buildPhase = ''
    echo "Hello from bash"
    make build
  '';
}"#;
        let html = highlight_code(Language::Nix, code);

        // Should contain Nix highlighting
        assert!(html.contains("class=\"hl-"));
        // Should contain the bash content
        assert!(html.contains("echo"));
        assert!(html.contains("make"));
        // String content should be present
        assert!(html.contains("Hello from bash"));
    }

    #[test]
    fn test_markdown_injection_rust() {
        // Markdown code block with embedded Rust should have full Rust highlighting
        let md = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let html = highlight_code(Language::Markdown, md);

        // All Rust tokens should be highlighted
        assert!(
            html.contains("hl-keyword"),
            "fn should be highlighted as keyword"
        );
        assert!(
            html.contains("hl-function"),
            "main/println should be highlighted as function"
        );
        assert!(
            html.contains("hl-string"),
            "string literal should be highlighted"
        );
        assert!(
            html.contains("hl-punctuation-bracket"),
            "brackets should be highlighted"
        );
    }
}
