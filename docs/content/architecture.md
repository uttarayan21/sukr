---
title: "Architecture"
description: "How sukr transforms markdown into zero-JS static sites"
weight: 5
toc: true
---

Sukr is a 12-module static site compiler. Every feature that would typically require client-side JavaScript is moved to build-time.

## Pipeline Overview

```mermaid
flowchart LR
    A[Walk content/] --> B[Parse frontmatter]
    B --> C[Stream markdown]
    C --> D{Code block?}
    D -->|Yes| E[Intercept]
    D -->|No| F[Pass through]
    E --> G[Render HTML]
    F --> G
    G --> H[Apply template]
    H --> I[Write public/]
```

## Module Responsibilities

| Module               | Purpose                                             |
| :------------------- | :-------------------------------------------------- |
| `main.rs`            | Pipeline orchestrator — wires everything together   |
| `config.rs`          | Loads `site.toml` configuration                     |
| `content.rs`         | Discovers sections, pages, and navigation structure |
| `render.rs`          | Markdown→HTML with code block interception          |
| `highlight.rs`       | Tree-sitter syntax highlighting (14 languages)      |
| `math.rs`            | KaTeX rendering to MathML                           |
| `mermaid.rs`         | Mermaid diagrams to inline SVG                      |
| `css.rs`             | CSS minification via lightningcss                   |
| `template_engine.rs` | Tera template loading and rendering                 |
| `feed.rs`            | Atom feed generation                                |
| `sitemap.rs`         | XML sitemap generation                              |
| `error.rs`           | Structured error types with source chaining         |

## The Interception Pattern

The core innovation is **event-based interception**. Rather than parsing markdown into an AST and walking it twice, sukr streams `pulldown-cmark` events and intercepts specific patterns:

```mermaid
flowchart TD
    A[pulldown-cmark event stream] --> B{Event type?}
    B -->|Start CodeBlock| C[Buffer content]
    C --> D{Language tag?}
    D -->|rust, python, etc.| E[Tree-sitter highlighting]
    D -->|mermaid| F[Mermaid SVG rendering]
    D -->|math delimiters| G[KaTeX MathML]
    D -->|unknown| H[HTML escape only]
    E --> I[Emit highlighted spans]
    F --> I
    G --> I
    H --> I
    B -->|Other events| J[Pass through]
    I --> K[Continue stream]
    J --> K
```

This pattern avoids buffering the entire document. Each code block is processed in isolation as it streams through.

## Why Zero-JS

Traditional SSGs ship JavaScript for:

| Feature             | Typical Approach       | Sukr Approach                                     |
| :------------------ | :--------------------- | :------------------------------------------------ |
| Syntax highlighting | Prism.js, Highlight.js | Tree-sitter at build-time → `<span class="hl-*">` |
| Math rendering      | MathJax, KaTeX.js      | KaTeX at build-time → MathML (browser-native)     |
| Diagrams            | Mermaid.js             | mermaid-rs at build-time → inline SVG             |
| Mobile nav          | JavaScript toggle      | CSS `:has()` + checkbox hack                      |

The result: **zero bytes of JavaScript** in the output. Pages load instantly, work without JS enabled, and avoid the complexity of client-side hydration.

## Static Configuration Pattern

Tree-sitter grammars are expensive to initialize. Sukr uses `LazyLock` to create each language configuration exactly once:

```rust
static RUST_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter_rust::LANGUAGE.into(),
        "rust",
        tree_sitter_rust::HIGHLIGHTS_QUERY,
        "",
    ).expect("query should be valid");
    config.configure(HIGHLIGHT_NAMES);
    config
});
```

This pattern ensures O(1) lookup per language regardless of how many code blocks exist in the site.

## Single-Pass Content Discovery

The `SiteManifest` struct aggregates all content in one filesystem traversal:

- Homepage (`_index.md` at root)
- Sections (directories with `_index.md`)
- Section items (posts, projects, pages)
- Navigation tree (derived from structure + frontmatter weights)

This avoids repeated directory scans during template rendering.

## Implementation Notes

Sukr prioritizes **output quality** over minimal build-time footprint. Current dependency choices reflect this:

| Feature  | Library    | Trade-off                                                                |
| :------- | :--------- | :----------------------------------------------------------------------- |
| Math     | KaTeX      | Full LaTeX coverage; heavier than minimal alternatives like latex2mathml |
| Diagrams | mermaid-rs | High-fidelity SVG; uses headless rendering under the hood                |

Lighter alternatives exist and may be evaluated as they mature. The goal is browser-native output with zero client-side JavaScript—build-time weight is a secondary concern.
