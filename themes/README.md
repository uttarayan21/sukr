# Syntax Highlighting Themes

This directory contains CSS themes for sukr's syntax highlighting system.

## Available Themes

| Theme              | Description                 |
| ------------------ | --------------------------- |
| `default.css`      | Dracula-inspired dark theme |
| `dracula.css`      | Classic Dracula colors      |
| `gruvbox.css`      | Warm retro palette          |
| `nord.css`         | Cool arctic colors          |
| `github_dark.css`  | GitHub's dark mode          |
| `github_light.css` | GitHub's light mode         |

## Usage

1. **Copy a theme** to your project's static directory
2. **Import it** in your main CSS file:

```css
@import "themes/default.css";
```

sukr uses [lightningcss](https://lightningcss.dev/) which bundles `@import` rules at build time—your final CSS will be a single minified file.

## Customization

Themes use CSS custom properties for easy customization. Override any variable in your own CSS:

```css
@import "themes/default.css";

/* Override just the keyword color */
:root {
  --hl-keyword: #e879f9;
}
```

### Core Variables

All themes define these variables in `:root`:

| Variable        | Description            |
| --------------- | ---------------------- |
| `--hl-keyword`  | Keywords, control flow |
| `--hl-string`   | String literals        |
| `--hl-function` | Function names         |
| `--hl-comment`  | Comments               |
| `--hl-type`     | Type names             |
| `--hl-number`   | Numeric literals       |
| `--hl-variable` | Variables              |
| `--hl-operator` | Operators              |

## Hierarchical Scopes

sukr generates **hierarchical CSS classes** for fine-grained styling:

```html
<span class="hl-keyword-control-return">return</span>
```

Themes can style at any level of specificity:

```css
/* All keywords */
.hl-keyword {
  color: var(--hl-keyword);
}

/* Just control-flow keywords */
.hl-keyword-control {
  color: #ff79c6;
}

/* Just return/break/continue */
.hl-keyword-control-return {
  font-weight: bold;
}
```

If a specific class isn't styled, highlighting falls back up the hierarchy.

## Creating Custom Themes

Start with `default.css` and modify the `:root` variables to create your own color scheme. The class rules reference these variables, so changing values updates the entire theme.

## Note

These themes are **not bundled into the sukr binary**—they're provided as starting points. Copy what you need to your project and customize to match your site's design.
