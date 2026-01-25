//! CSS processing via lightningcss.

use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};

/// Minify CSS content.
///
/// Returns minified CSS string on success, or the original input on error.
pub fn minify_css(css: &str) -> String {
    match try_minify(css) {
        Ok(minified) => minified,
        Err(_) => css.to_string(),
    }
}

fn try_minify(css: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default())
        .map_err(|e| format!("parse error: {:?}", e))?;

    stylesheet
        .minify(MinifyOptions::default())
        .map_err(|e| format!("minify error: {:?}", e))?;

    let result = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            ..Default::default()
        })
        .map_err(|e| format!("print error: {:?}", e))?;

    Ok(result.code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minify_removes_whitespace() {
        let input = r#"
            .foo {
                color: red;
            }
        "#;
        let output = minify_css(input);

        // Should be smaller (whitespace removed)
        assert!(output.len() < input.len());
        // Should still contain the essential content
        assert!(output.contains(".foo"));
        assert!(output.contains("color"));
        assert!(output.contains("red"));
    }

    #[test]
    fn test_minify_removes_comments() {
        let input = r#"
            /* This is a comment */
            .bar { background: blue; }
        "#;
        let output = minify_css(input);

        // Comment should be removed
        assert!(!output.contains("This is a comment"));
        // Rule should remain
        assert!(output.contains(".bar"));
    }

    #[test]
    fn test_minify_merges_selectors() {
        let input = r#"
            .foo { color: red; }
            .bar { color: red; }
        "#;
        let output = minify_css(input);

        // Should merge identical rules
        // Either ".foo,.bar" or ".bar,.foo" pattern
        assert!(output.contains(","));
    }
}
