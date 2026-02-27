---
"@biomejs/biome": minor
---

Added new nursery lint rule [`noUndeclaredStyles`](https://biomejs.dev/linter/rules/no-undeclared-styles/) for HTML. The rule detects CSS class names used in `class="..."` attributes that are not defined in any `<style>` block or linked stylesheet within the same file.

```html
<!-- .typo is used but never defined -->
<html>
  <head>
    <style>.button { color: blue; }</style>
  </head>
  <body>
    <div class="button typo"></div>
  </body>
</html>
```
