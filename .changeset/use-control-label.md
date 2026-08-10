---
"@biomejs/biome": patch
---

Added a new nursery rule [`useControlLabel`](https://biomejs.dev/linter/rules/use-control-label/) for [`#8510`](https://github.com/biomejs/biome/issues/8510), which reports interactive control elements (`button`, `menuitem`) that have no accessible label, for both HTML and JSX:

```jsx
<button />
```
