---
"@biomejs/biome": minor
---

Added the new rule [`noNestedComponentDefinitions`](https://biomejs.dev/linter/rules/no-nested-component-definitions),
which disallows nested component definitions in React components.

This rule is useful for preventing potential performance issues and improving code readability by ensuring that components are defined at the top level.

**Example (Invalid):**

```jsx
function ParentComponent() {
  function ChildComponent() {
    return <div>Hello</div>;
  }
  return <ChildComponent />;
}
```

**Example (Valid):**

```jsx
function ChildComponent() {
  return <div>Hello</div>;
}
function ParentComponent() {
  return <ChildComponent />;
}
```
