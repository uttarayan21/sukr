---
title: Content Organization
description: How the filesystem maps to your site structure
weight: 1
---

# Content Organization

sukr builds your site structure from your `content/` directory. No routing config needed — the filesystem _is_ the config.

## The Rule

```text
content/foo/bar.md  →  public/foo/bar.html
content/about.md    →  public/about.html
content/_index.md   →  public/index.html
```

That's it. Paths mirror exactly, with `.md` becoming `.html`.

## Directory Layout

```text
content/
├── _index.md           # Homepage (required)
├── about.md            # → /about.html
├── contact.md          # → /contact.html
├── blog/               # Section directory
│   ├── _index.md       # → /blog/index.html (section index)
│   ├── first-post.md   # → /blog/first-post.html
│   └── second-post.md  # → /blog/second-post.html
└── projects/
    ├── _index.md       # → /projects/index.html
    └── my-app.md       # → /projects/my-app.html
```

## What Makes a Section

A section is any directory containing `_index.md`. This file:

1. Provides metadata for the section (title, description)
2. Triggers section listing behavior
3. Appears in the navigation

Directories without `_index.md` are ignored.

## Navigation Generation

Navigation builds automatically from:

- **Top-level `.md` files** (except `_index.md`) → page links
- **Directories with `_index.md`** → section links

Items sort by `weight` in frontmatter (lower first), then alphabetically.

```yaml
---
title: Blog
weight: 10 # Appears before items with weight > 10
---
```

## URL Examples

| Source Path              | Output Path              | URL                |
| ------------------------ | ------------------------ | ------------------ |
| `content/_index.md`      | `public/index.html`      | `/`                |
| `content/about.md`       | `public/about.html`      | `/about.html`      |
| `content/blog/_index.md` | `public/blog/index.html` | `/blog/`           |
| `content/blog/hello.md`  | `public/blog/hello.html` | `/blog/hello.html` |

## Key Points

- No config files for routing
- Directory names become URL segments
- `_index.md` = section index, not a regular page
- Flat output structure (no nested `index.html` per page)
