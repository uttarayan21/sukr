---
title: Syntax Highlighting
description: Build-time code highlighting with Tree-sitter
weight: 3
---

# Syntax Highlighting

sukr highlights code blocks at build time using Tree-sitter. No client-side JavaScript required.

## Usage

Use fenced code blocks with a language identifier:

````md
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
| C          | `c`                   |

## Examples

### Rust

```rust
fn main() {
    println!("Hello, world!");
}
```

### Python

```python
def greet(name: str) -> str:
    return f"Hello, {name}!"
```

### JavaScript

```javascript
const greet = (name) => `Hello, ${name}!`;
```

### TypeScript

```typescript
function greet(name: string): string {
  return `Hello, ${name}!`;
}
```

### Go

```go
func main() {
    fmt.Println("Hello, world!")
}
```

### Bash

```bash
#!/bin/bash
echo "Hello, $USER!"
```

### Nix

```nix
{ pkgs }:
pkgs.mkShell { buildInputs = [ pkgs.hello ]; }
```

### TOML

```toml
[package]
name = "sukr"
version = "0.1.0"
```

### YAML

```yaml
name: Build
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
```

### JSON

```json
{
  "name": "sukr",
  "version": "0.1.0"
}
```

### HTML

```html
<!DOCTYPE html>
<html>
  <body>
    Hello!
  </body>
</html>
```

### CSS

```css
.container {
  display: flex;
  color: #ff79c6;
}
```

### C

```c
#include <stdio.h>
int main() {
    printf("Hello!\n");
    return 0;
}
```

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

## Injection Support

Some languages support injection—highlighting embedded languages. For example, Bash inside Nix strings:

```nix
stdenv.mkDerivation {
  buildPhase = ''
    echo "Building..."
    make -j$NIX_BUILD_CORES
  '';
}
```

Markdown also supports injection—code blocks inside markdown fences are highlighted with their respective languages.

## Fallback

Unknown languages fall back to plain `<code>` blocks without highlighting.
