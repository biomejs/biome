---
"@biomejs/biome": patch
---

Added a new nursery rule [`noComponentHookFactories`](https://biomejs.dev/linter/rules/no-component-hook-factories/), that disallows defining React components or custom hooks inside other functions.

For example, the following snippets trigger the rule:

```jsx
function createComponent(label) {
  function MyComponent() {
    return <div>{label}</div>;
  }
  return MyComponent;
}
```

```jsx
function Parent() {
  function Child() {
    return <div />;
  }
  return <Child />;
}
```
