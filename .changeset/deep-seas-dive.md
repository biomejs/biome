---
"@biomejs/biome": patch
---

Fix [#342](https://github.com/biomejs/biome/issues/342), js parser handle unterminated `JSX_STRING_LITERAL` properly

```jsx
function Comp() {
  return (
      <a rel="
```
