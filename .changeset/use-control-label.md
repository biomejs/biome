---
"@biomejs/biome": patch
---

Added a new nursery rule [`useControlLabel`](https://biomejs.dev/linter/rules/use-control-label/) for both HTML and JSX, which reports interactive control elements (`button`, `menuitem`) without an accessible label.

```jsx
<button />
```
