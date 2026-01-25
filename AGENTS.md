# Project Agent Configuration

## Predicates

This project uses [predicate](https://github.com/nrdxp/predicate) for agent configuration.

**Installation Location:** `.agent/predicates/`

The agent MUST read and adhere to the global engineering ruleset and any active fragments:

```
.agent/
├── predicates/
│   ├── global.md              # Base engineering ruleset (required)
│   └── fragments/             # Active extensions
│       └── rust.md            # Rust conventions
└── workflows/
    └── ...                    # Task-specific workflows
```

**Active Fragments:**

- Rust idioms (`.agent/predicates/fragments/rust.md`)
- DepMap MCP tools (`.agent/predicates/fragments/depmap.md`)

**Available Workflows:**

- `/ai-audit` — Audit code for AI-generated patterns
- `/core` — C.O.R.E. structured interaction protocol
- `/predicate` — Re-read global rules; combats context drift
- `/humanizer` — Remove AI writing patterns from text

---

## Project Overview

**nrd.sh** is a bespoke static site compiler written in Rust. It transforms Markdown content into a minimal, high-performance static site with zero client-side dependencies.

### Philosophy

- **Suckless:** No bloated runtimes, no unnecessary JavaScript
- **Hermetic:** Single binary with all dependencies compiled in
- **Elegant:** State-of-the-art syntax highlighting via Tree-sitter

### Architecture

The compiler implements an **Interceptor Pipeline**:

1. **Ingest:** Walk `content/`, parse TOML frontmatter
2. **Stream:** Feed Markdown to `pulldown-cmark` event parser
3. **Intercept:** Route code blocks to Tree-sitter, Mermaid, KaTeX
4. **Render:** Push modified events to HTML writer
5. **Layout:** Wrap in `maud` templates
6. **Write:** Output to `public/`

---

## Build & Commands

```bash
# Development
nix develop          # Enter dev shell with Rust toolchain
cargo build          # Build compiler
cargo run            # Run compiler (builds site to public/)

# Production
nix build            # Build hermetic release binary
./result/bin/nrd-sh  # Run release compiler
```

---

## Code Style

- Rust 2024 edition
- Follow `.agent/predicates/fragments/rust.md` conventions
- Prefer standard library over external crates
- No `unwrap()` in library code; use proper error handling

---

## Architecture

```
.
├── Cargo.toml           # Rust manifest
├── flake.nix            # Nix build environment
├── src/
│   ├── main.rs          # Pipeline orchestrator
│   ├── highlight.rs     # Tree-sitter highlighting
│   ├── render.rs        # Pulldown-cmark interception
│   └── assets.rs        # CSS processing
├── queries/             # Tree-sitter S-expression queries
├── content/             # Blog content (Markdown + TOML frontmatter)
├── theme/               # Maud templates and CSS
└── deprecated/          # Legacy Zola infrastructure (archived)
```

---

## Testing

- Test runner: `cargo test`
- Naming: `test_<scenario>_<expected_outcome>`
- Focus on content transformation correctness

---

## Security

- No user input at runtime (build-time only)
- Validate frontmatter schema during parsing
- No secrets in content or templates

---

## Configuration

Site configuration will be defined in `site.toml` (future):

```toml
base_url = "https://nrd.sh/"
title = "nrdxp"
# ...
```

Currently using hardcoded defaults during initial development.
