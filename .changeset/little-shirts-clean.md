---
"@biomejs/biome": patch
---

Added the nursery rule [`noDisabledAttribute`](https://biomejs.dev/linter/rules/no-disabled-attribute/), which warns when buttons and other form controls use the native `disabled` attribute. A disabled control can't be reached with the Tab key, so the rule suggests `aria-disabled` or `readonly` instead.

```jsx
<button disabled>Save</button>
<input type="email" disabled />
```
