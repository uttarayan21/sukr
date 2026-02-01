---
title: Configuration
description: Complete reference for site.toml configuration
weight: 2
---

sukr is configured via `site.toml`. All settings have sensible defaults.

## Basic Configuration

```toml
title    = "My Site"
author   = "Your Name"
base_url = "https://example.com"
```

| Field      | Required | Description                      |
| ---------- | -------- | -------------------------------- |
| `title`    | Yes      | Site title (used in page titles) |
| `author`   | Yes      | Author name (used in feeds)      |
| `base_url` | Yes      | Canonical URL for the site       |

## Path Configuration

All paths are optional. Default values shown:

```toml
[paths]
content   = "content"    # Markdown source files
output    = "public"     # Generated HTML output
static    = "static"     # Static assets (copied as-is)
templates = "templates"  # Tera template files
```

Paths are resolved **relative to the config file location**. This enables monorepo setups:

```bash
# Build site from subdirectory
sukr -c sites/blog/site.toml
# Paths resolve relative to sites/blog/
```

## CLI Options

```bash
sukr                           # Use ./site.toml
sukr -c path/to/site.toml      # Custom config
sukr --config path/to/site.toml
sukr -h, --help                # Show help
```

## Frontmatter

Each Markdown file can have YAML frontmatter:

```yaml
---
title: Page Title
description: Optional description
date: 2024-01-15 # For blog posts
weight: 10 # Sort order (lower = first)
nav_label: Short Name # Override nav display
section_type: blog # Override section template
template: custom # Override page template
---
```

### Section Types

The `section_type` field determines which template is used for section indexes:

- `blog` → `templates/section/blog.html`
- `projects` → `templates/section/projects.html`
- _(any other)_ → `templates/section/default.html`

If not specified, sukr uses the directory name as the section type.
