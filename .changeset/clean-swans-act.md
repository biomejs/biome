---
"@biomejs/biome": patch
---

Added the nursery rule [`noDuplicateSpread`](https://biomejs.dev/linter/rules/no-duplicate-spread/). Disallow JSX prop spreading the same identifier multiple times.

**Invalid:**

```jsx
<div {...props} something="else" {...props} />
```
