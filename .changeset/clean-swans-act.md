---
"@biomejs/biome": patch
---

Added the nursery rule [`noDuplicatedSpreadProps`](https://biomejs.dev/linter/rules/no-duplicated-spread-props/). Disallow JSX prop spreading the same identifier multiple times.

**Invalid:**

```jsx
<div {...props} something="else" {...props} />
```
