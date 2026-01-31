---
title: Syntax Highlighting
description: Build-time code highlighting with Tree-sitter
weight: 3
---

# Syntax Highlighting

sukr highlights code blocks at build time using Tree-sitter. No client-side JavaScript required.

## Usage

Use fenced code blocks with a language identifier:

````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
````

## Supported Languages

| Language   | Identifier            |
| ---------- | --------------------- |
| Rust       | `rust`, `rs`          |
| Python     | `python`, `py`        |
| JavaScript | `javascript`, `js`    |
| TypeScript | `typescript`, `ts`    |
| Go         | `go`, `golang`        |
| Bash       | `bash`, `sh`, `shell` |
| Nix        | `nix`                 |
| TOML       | `toml`                |
| YAML       | `yaml`, `yml`         |
| JSON       | `json`                |
| HTML       | `html`                |
| CSS        | `css`                 |
| Markdown   | `markdown`, `md`      |

## How It Works

1. During Markdown parsing, code blocks are intercepted
2. Tree-sitter parses the code and generates a syntax tree
3. Spans are generated with semantic CSS classes
4. All work happens at build time

## Styling

Highlighted code uses semantic CSS classes:

```css
.keyword {
  color: #ff79c6;
}
.string {
  color: #f1fa8c;
}
.function {
  color: #50fa7b;
}
.comment {
  color: #6272a4;
}
.number {
  color: #bd93f9;
}
```

The exact classes depend on the language grammar.

## Nix Language Support

sukr includes full Nix highlighting with injection support. Bash code inside `buildPhase` and similar attributes is highlighted correctly:

```nix
stdenv.mkDerivation {
  buildPhase = ''
    echo "Building..."
    make -j$NIX_BUILD_CORES
  '';
}
```

## Fallback

Unknown languages fall back to plain `<code>` blocks without highlighting.
