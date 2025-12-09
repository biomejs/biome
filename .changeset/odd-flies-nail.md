---
"@biomejs/biome": minor
---

Improved the CSS parser for CSS modules. When Biome encounters a `*.module.css` file, enables the CSS modules feature.

If your codebase has only `*.module.css` files, you can disable the parser feature as follows:

```diff
{
  "css": {
    "parser": {
-      "cssModules": true
    }
  }
}
```
