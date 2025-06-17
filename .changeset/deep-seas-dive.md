---
"@biomejs/biome": patch
---

Fixed [#342](https://github.com/biomejs/biome/issues/342): The JavaScript parser now properly handles unterminated string literals, such as:

```jsx
function Comp() {
  return (
      <a rel="
```
