---
"@biomejs/biome": patch
---

Fixed [#8027](https://github.com/biomejs/biome/issues/8027). [`useReactFunctionComponents`](https://biomejs.dev/linter/rules/use-react-function-components/) no longer reports class components that implement `componentDidCatch` using class expressions.

The rule now correctly recognizes error boundaries defined as class expressions:
```jsx
const ErrorBoundary = class extends Component {
  componentDidCatch(error, info) {}

  render() {
    return this.props.children;
  }
}
```
