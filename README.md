# sukr

**Minimal static site compiler — suckless, Rust, zero JS.**

sukr transforms Markdown content into high-performance static HTML. No bloated runtimes, no unnecessary JavaScript, just clean output.

## Features

- **Build-time rendering** — Syntax highlighting via Tree-sitter, no client JS
- **Tera templates** — Runtime customizable, no recompilation needed
- **Convention over configuration** — Add sections by creating directories
- **Monorepo support** — Multiple sites via `-c` flag

## Quick Start

```bash
# Build
cargo build --release

# Run (uses ./site.toml)
sukr

# Custom config (monorepo)
sukr -c sites/blog/site.toml

# Help
sukr --help
```

## Configuration

Create `site.toml`:

```toml
title    = "My Site"
author   = "Your Name"
base_url = "https://example.com"

[paths]  # All optional
content   = "content"    # default
output    = "public"     # default
static    = "static"     # default
templates = "templates"  # default
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

## License

MIT
