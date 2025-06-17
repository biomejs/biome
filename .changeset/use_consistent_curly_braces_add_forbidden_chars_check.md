---
"@biomejs/biome": patch
---


Fixed [`useConsistentCurlyBraces breaks react/no-unescaped-entities rule`](https://github.com/biomejs/biome/issues/5391)

Added a check for forbidden characters: `>`, `"`, `'` and `}`.
If any of these characters are detected, curly braces will be preserved.

Example:

```jsx
function MyComponent() {
  return <Foo>Jupiter {'>'} Venus</Foo>;
}
```
