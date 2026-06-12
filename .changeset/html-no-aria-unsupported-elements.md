---
"@biomejs/biome": minor
---

Added the HTML lint rule [`noAriaUnsupportedElements`](https://biomejs.dev/linter/rules/no-aria-unsupported-elements/). This rule enforces that elements that do not support ARIA roles, states, and properties (`meta`, `html`, `script`, `style`) do not have `role` or `aria-*` attributes.

```html
<!-- Invalid: meta does not support aria attributes -->
<meta charset="UTF-8" role="meta" />
```
