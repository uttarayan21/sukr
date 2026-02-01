---
title: Sections
description: Automatic section discovery and processing
weight: 2
---

sukr automatically discovers sections from your content directory structure.

## What is a Section?

A section is any directory under `content/` that contains an `_index.md` file:

```text
content/
├── _index.md           # Homepage (not a section)
├── about.md            # Standalone page
├── blog/               # ← This is a section
│   ├── _index.md       # Section index
│   └── my-post.md      # Section content
└── projects/           # ← This is also a section
    ├── _index.md
    └── project-a.md
```

## Section Discovery

sukr automatically:

1. Scans `content/` for directories with `_index.md`
2. Collects all `.md` files in that directory (excluding `_index.md`)
3. Renders the section index template with the items
4. Renders individual content pages (for blog-type sections)

## Section Types

The section type determines which template is used. It's resolved in order:

1. **Frontmatter override**: `section_type: blog` in `_index.md`
2. **Directory name**: `content/blog/` → type `blog`

### Built-in Section Types

| Type       | Behavior                                               |
| ---------- | ------------------------------------------------------ |
| `blog`     | Sorts by date (newest first), renders individual posts |
| `projects` | Sorts by weight, card-style listing                    |
| _(other)_  | Sorts by weight, uses default template                 |

## Section Frontmatter

In `_index.md`:

```yaml
---
title: My Blog
description: Thoughts and tutorials
section_type: blog # Optional, defaults to directory name
weight: 1 # Nav order
---
```

## Adding a New Section

1. Create directory: `content/recipes/`
2. Create index: `content/recipes/_index.md`
3. Add content: `content/recipes/pasta.md`
4. Optionally create template: `templates/section/recipes.html`

That's it. sukr handles the rest.
