---
"@biomejs/biome": minor
---

Added the [`useSemanticElements`](https://biomejs.dev/linter/rules/use-semantic-elements/) lint rule for HTML. The rule now detects the use of `role` attributes in HTML elements and suggests using semantic elements instead.

For example, the following code is now flagged:

```html
<div role="navigation"></div>
```

The rule suggests using `<nav>` instead.
