---
"@biomejs/biome": patch
---

Added the nursery rule [`noDisabledButton`](https://biomejs.dev/linter/rules/no-disabled-button/), which warns when button-like elements use `disabled` instead of `aria-disabled`.

```jsx
<button disabled>Save</button>
```
