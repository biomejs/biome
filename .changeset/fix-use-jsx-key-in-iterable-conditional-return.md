---
"@biomejs/biome": patch
---

Fixed [`useJsxKeyInIterable`](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable/) reporting elements assigned to variables that are only used as JSX children, for example when the callback returns a conditional expression. The following code is no longer reported:

```jsx
xs.map((x) => {
  const inner = <b>{x}</b>;
  return f ? <li key={x}>{inner}</li> : <p key={x}>{inner}</p>;
});
```
