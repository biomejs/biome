---
"@biomejs/biome": patch
---

Added the new nursery rule [`noReactObjectTypeAsDefaultProp`](https://biomejs.dev/linter/rules/no-react-object-type-as-default-prop/), which disallows array, object, and function values as default props in React components.

For example, the following snippet triggers the rule.

```jsx
function Component({ items = [] }) {
  return items;
}
```

