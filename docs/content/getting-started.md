---
title: Getting Started
description: Install sukr and build your first site
weight: 0
---

This guide walks you through installing sukr and creating your first static site.

## Installation

### From source (recommended)

```bash
git clone https://github.com/nrdxp/sukr
cd sukr
cargo install --path .
```

### With Nix

```bash
nix build github:nrdxp/sukr
./result/bin/sukr --help
```

## Create Your First Site

### 1. Create directory structure

```bash
mkdir my-site && cd my-site
mkdir -p content templates static
```

### 2. Create configuration

Create `site.toml`:

```toml
title    = "My Site"
author   = "Your Name"
base_url = "https://example.com"
```

### 3. Create homepage

Create `content/_index.md`:

```markdown
---
title: Welcome
description: My awesome site
---

# Hello, World!

This is my site built with sukr.
```

### 4. Create templates

Copy the default templates from the sukr repository, or create your own Tera templates.

### 5. Build

```bash
sukr
```

Your site is now in `public/`.

## Next Steps

- Learn about [Configuration](configuration.html)
- Explore [Features](features/index.html)
