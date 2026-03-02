---
"@biomejs/biome": minor
---

Added new nursery lint rule [`noUndeclaredClasses`](https://biomejs.dev/linter/rules/no-undeclared-classes/) for HTML, JSX, and SFC files (Vue, Astro, Svelte). The rule detects CSS class names used in `class="..."` (or `className`) attributes that are not defined in any `<style>` block or linked stylesheet reachable from the file.

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
