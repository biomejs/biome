---
"@biomejs/biome": minor
---

Improved the CSS parser for CSS modules. Biome now automatically enables CSS modules parsing for `*.module.css` files.

If your codebase has only `*.module.css` files, you can remove the parser feature as follows, because now Biome does it for you:

```diff
{
  "css": {
    "parser": {
-      "cssModules": true
    }
  }
}
```
