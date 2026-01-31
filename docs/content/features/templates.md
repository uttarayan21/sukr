---
title: Tera Templates
description: Customizable templates without recompilation
weight: 1
---

# Tera Templates

sukr uses [Tera](https://tera.netlify.app/), a Jinja2-like templating engine. Templates are loaded at runtime, so you can modify them without recompiling sukr.

## Template Directory Structure

```text
templates/
├── base.html               # Shared layout (required)
├── page.html               # Standalone pages
├── homepage.html           # Site homepage
├── section/
│   ├── default.html        # Fallback section index
│   ├── blog.html           # Blog section index
│   └── projects.html       # Projects section index
└── content/
    ├── default.html        # Fallback content page
    └── post.html           # Blog post
```

## Template Inheritance

All templates extend `base.html`:

```html
{% extends "base.html" %} {% block content %}
<article>
  <h1>{{ page.title }}</h1>
  {{ content | safe }}
</article>
{% endblock content %}
```

## Available Context Variables

### All Templates

| Variable        | Description                     |
| --------------- | ------------------------------- |
| `config.title`  | Site title                      |
| `config.author` | Site author                     |
| `nav`           | Navigation items                |
| `page_path`     | Current page path               |
| `prefix`        | Relative path prefix for assets |
| `base_url`      | Canonical base URL              |
| `title`         | Current page title              |

### Page Templates

| Variable           | Description           |
| ------------------ | --------------------- |
| `page.title`       | Page title            |
| `page.description` | Page description      |
| `content`          | Rendered HTML content |

### Section Templates

| Variable              | Description                       |
| --------------------- | --------------------------------- |
| `section.title`       | Section title                     |
| `section.description` | Section description               |
| `items`               | Array of content items in section |

### Content Item Fields (in `items`)

| Variable           | Description         |
| ------------------ | ------------------- |
| `item.title`       | Content title       |
| `item.description` | Content description |
| `item.date`        | Publication date    |
| `item.path`        | URL path            |
| `item.slug`        | URL slug            |

## Template Override

Set `template` in frontmatter to use a custom template:

```yaml
---
title: Special Page
template: special
---
```

This uses `templates/page/special.html` instead of the default.
