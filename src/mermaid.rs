//! Mermaid diagram rendering via mermaid-rs-renderer.
//!
//! Converts Mermaid diagram definitions to SVG at build-time.

use mermaid_rs_renderer::RenderOptions;

/// Render a Mermaid diagram to SVG.
///
/// # Arguments
/// * `code` - The Mermaid diagram definition
///
/// # Returns
/// The rendered SVG string, or an error message on failure.
pub fn render_diagram(code: &str) -> Result<String, String> {
    let opts = RenderOptions::modern();
    mermaid_rs_renderer::render_with_options(code, opts).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_flowchart() {
        let result = render_diagram("flowchart LR; A-->B-->C").unwrap();
        assert!(result.contains("<svg"));
        assert!(result.contains("</svg>"));
    }

    #[test]
    fn test_sequence_diagram() {
        let result = render_diagram("sequenceDiagram\n    Alice->>Bob: Hello").unwrap();
        assert!(result.contains("<svg"));
    }

    #[test]
    fn test_invalid_syntax_no_panic() {
        // Should not panic, returns error
        let result = render_diagram("invalid diagram syntax ???");
        // May succeed with error node or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }
}
