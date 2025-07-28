---
"@biomejs/biome": patch
---

Added the nursery rule [`useReactFunctionComponents`](https://biomejs.dev/linter/rules/use-react-function-components/). This rule enforces the preference to use function components instead of class components.

Valid:
```jsx
function Foo() {
  return <div>Hello, world!</div>;
}
```

Invalid:
```jsx
class Foo extends React.Component {
  render() {
    return <div>Hello, world!</div>;
  }
}
```
