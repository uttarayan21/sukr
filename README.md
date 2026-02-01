<p align="center">
  <img src="docs/static/logo.png" alt="sukr logo" width="128" />
</p>

# sukr

**Minimal static site compiler — suckless, Rust, zero JS.**

sukr transforms Markdown content into high-performance static HTML. No bloated runtimes, no client-side JavaScript, just clean output.

## Why sukr?

Most static site generators punt rich content to the browser. sukr doesn't.

- **Tree-sitter syntax highlighting** — Proper parsing, not regex. Supports language injection (Nix shells, HTML scripts).
- **Build-time math** — KaTeX renders LaTeX to static HTML. No 300KB JavaScript bundle.
- **Build-time diagrams** — Mermaid compiles to inline SVG. Diagrams load instantly.

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

## Comparison

| Feature             |    sukr     |  Zola   |  Hugo  | Eleventy |
| :------------------ | :---------: | :-----: | :----: | :------: |
| Syntax Highlighting | Tree-sitter | syntect | Chroma | Plugins  |
| Build-time Math     |     ✅      |   ❌    |   ❌   |  Plugin  |
| Build-time Diagrams |     ✅      |   ❌    |   ❌   |  Plugin  |
| Zero JS Output      |     ✅      |   ❌    |   ❌   | Optional |
| Single Binary       |     ✅      |   ✅    |   ✅   |    ❌    |

See the [full comparison](https://sukr.io/comparison.html) for details.

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
├── _index.md              # Homepage
├── getting-started.md     # Page → /getting-started.html
├── configuration.md       # Page → /configuration.html
└── features/
    ├── _index.md          # Section index → /features/index.html
    └── templates.md       # Page → /features/templates.html
...
```

## Documentation

Full documentation at [sukr.io](https://sukr.io) (built with sukr).

## License

MIT
