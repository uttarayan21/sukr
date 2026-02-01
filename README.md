<p align="center">
  <img src="docs/static/logo.png" alt="sukr logo" width="128" />
</p>

# sukr

**Minimal static site compiler — suckless, Rust, zero JS.**

sukr transforms Markdown content into high-performance static HTML. No bloated runtimes, no client-side JavaScript, just clean output.

## Features

- **Syntax highlighting** — Tree-sitter with language injection (Nix→Bash, HTML→JS/CSS)
- **Math rendering** — LaTeX to HTML via KaTeX at build time
- **Mermaid diagrams** — Rendered to inline SVG, no client JS
- **Tera templates** — Customize without recompiling
- **Hierarchical navigation** — Nested sections with table of contents
- **Atom feeds** — Auto-generated for blog sections
- **Sitemap** — SEO-ready XML sitemap
- **CSS minification** — LightningCSS optimization
- **Monorepo support** — Multiple sites via `-c` flag

## Quick Start

```bash
# Build
cargo build --release

# Run (uses ./site.toml)
sukr

# Custom config (monorepo)
sukr -c docs/site.toml
```

## Configuration

Create `site.toml`:

```toml
title    = "My Site"
author   = "Your Name"
base_url = "https://example.com"

[paths]  # All optional, defaults shown
content   = "content"
output    = "public"
static    = "static"
templates = "templates"

[nav]  # Optional
nested = false  # Show section children in nav
toc    = false  # Enable table of contents
```

## Content Structure

```
content/
├── _index.md          # Homepage
├── about.md           # Standalone page → /about.html
└── blog/
    ├── _index.md      # Section index → /blog/index.html
    └── my-post.md     # Post → /blog/my-post.html
```

## Documentation

Full documentation at [sukr.io](https://sukr.io) (built with sukr).

## License

MIT
