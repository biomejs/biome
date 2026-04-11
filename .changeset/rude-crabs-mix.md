---
"@biomejs/biome": patch
---

Added the nursery rule [`noJsxLeakedDollar`](https://biomejs.dev/linter/rules/no-jsx-leaked-dollar), which disallows a leaked `$` sign before a JSX expression.

**Invalid**:

```jsx
function MyComponent({ user }) {
  return <div>Hello ${user.name}</div>;
}
```
