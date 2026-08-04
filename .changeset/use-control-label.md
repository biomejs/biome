---
"@biomejs/biome": patch
---

Added a new nursery rule [`useControlLabel`](https://biomejs.dev/linter/rules/use-control-label/), which reports interactive control elements (`button`, `menuitem`) that have no accessible label, for both HTML and JSX:

```jsx
<button />
```
