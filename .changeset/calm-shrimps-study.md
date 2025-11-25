---
'@biomejs/biome': patch
---

Added the new rule [`noLeakedRender`](https://biomejs.dev/linter/rules/no-leaked-render). This rule helps prevent potential leaks when rendering components that use binary expressions or ternaries.

For example, the following code triggers the rule because the component would render `0`:

```jsx
const Component = () => {
 const count = 0;
 return <div>{count && <span>Count: {count}</span>}</div>;
}
```
