---
title: sukr
description: Minimal static site compiler — suckless, Rust, zero JS
---

# Welcome to sukr

**sukr** transforms Markdown into high-performance static HTML. No bloated runtimes, no client-side JavaScript, just clean output.

## Why sukr?

- **Fast builds** — Single Rust binary, parallel processing
- **Zero JS** — Syntax highlighting at build time via Tree-sitter
- **Flexible templates** — Runtime Tera templates, no recompilation
- **Monorepo-ready** — Multiple sites via `-c` config flag

## Quick Start

```bash
# Install
cargo install sukr

# Create site structure
mkdir -p content templates static
echo 'title = "My Site"' > site.toml
echo 'author = "Me"' >> site.toml
echo 'base_url = "https://example.com"' >> site.toml

# Build
sukr
```

## Documentation

Browse the sidebar for detailed documentation on all features.
