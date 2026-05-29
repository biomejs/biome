---
"@biomejs/biome": patch
---

Added the nursery rule [`useFunctionComponentDefinition`](https://biomejs.dev/linter/rules/use-function-component-definition/), which enforces a consistent function type for named React function components.

For example, the following snippet triggers the rule by default.

```jsx
const MyComponent = (props) => {
  return <div>{props.name}</div>;
};
```
