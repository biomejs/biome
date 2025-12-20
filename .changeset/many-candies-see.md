---
"@biomejs/biome": patch
---

Added the new nursery rule [`noJsxPropsBind`](https://biomejs.dev/linter/rules/no-jsx-props-bind). This rule disallows .bind(), arrow functions, or function expressions in JSX props.

**Invalid:**

```jsx
<Foo onClick={() => console.log('Hello!')}></Foo>
```
