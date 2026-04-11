---
"@biomejs/biome": patch
---

Added the nursery rule [`noJsxLeakedDollar`](https://biomejs.dev/linter/rules/no-jsx-leaked-dollar), which disallows a trailing `$` in a text node if the next sibling node is a JSX expression. This could be an unintentional mistake, resulting in a '$' being rendered as text in the output.

**Invalid**:

```jsx
function MyComponent({ user }) {
  return <div>Hello ${user.name}</div>;
}
```
