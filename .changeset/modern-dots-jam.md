---
"@biomejs/biome": minor
---

Added the new rule [`noReactPropAssign`](https://biomejs.dev/linter/rules/no_react_prop_assign), based on the react-hooks rule [react-hooks/react-compiler](https://www.npmjs.com/package/eslint-plugin-react-hooks)

The following code is now reported as invalid:

```jsx
function Foo(props) {
  props.bar = `Hello ${props.bar}`;
  return <div>{props.bar}</div>
}
```

The following code is now reported as valid:

```jsx
function Foo({bar}) {
  bar = `Hello ${bar}`;
  return <div>{bar}</div>
}
```
