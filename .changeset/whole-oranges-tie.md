---
"@biomejs/biome": patch
---

Added the nursery rule [`noNonScalableViewport`](https://biomejs.dev/linter/rules/no-non-scalable-viewport), which reports viewport metadata that disables user scaling with `user-scalable=no`.

For example:

```html
<meta name="viewport" content="width=device-width, user-scalable=no" />
```
