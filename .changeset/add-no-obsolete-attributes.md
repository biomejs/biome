---
"@biomejs/biome": patch
---

Added the new nursery rule [`noObsoleteAttributes`](https://biomejs.dev/linter/rules/no-obsolete-attributes/), which disallows obsolete attributes on HTML elements and suggests what to use instead.

```html
<table bgcolor="#ffffff" cellpadding="5"></table>
```
