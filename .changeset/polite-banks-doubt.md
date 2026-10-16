---
"@biomejs/biome": patch
---

Fixed parsing and formatting of multi-component defaults in native CSS `@function` parameters, such as `--padding: 8px 16px`, in CSS and SCSS. Native defaults preserve literal Sass-looking tokens and support explicit SCSS interpolation.
