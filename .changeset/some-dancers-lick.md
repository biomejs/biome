---
"@biomejs/biome": minor
---

Added the HTML lint rule [`useAriaPropsSupportedByRole`](https://biomejs.dev/linter/rules/use-aria-props-supported-by-role/), which enforces that ARIA properties are valid for the roles that are supported by the element.

```html
<a href="#" aria-checked></a>
```
