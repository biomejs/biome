---
"@biomejs/biome": minor
---

Added a new CSS parser option `tailwindDirectives`. Enabling this option will allow all of Tailwind v4's syntax additions to be parsed and formatted by Biome.

You can enable this by setting `css.parser.tailwindDirectives` to `true` in your Biome configuration.

```json
{
  "css": {
    "parser": {
      "tailwindDirectives": true
    }
  }
}
```
