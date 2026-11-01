---
"@biomejs/biome": patch
---

Fixed [`useJsxKeyInIterable`](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable/) reporting elements assigned to variables when the callback returns a conditional expression whose branches all have a key. The following code is no longer reported:

```jsx
xs.map((x) => {
  const inner = <b>{x}</b>;
  return f ? <li key={x}>{inner}</li> : <p key={x}>{inner}</p>;
});
```

The rule now also reports elements without a key returned early from an `if` block, when the last return has a key.
