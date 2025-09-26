---
"@biomejs/biome": patch
---


Added the [`useQwikValidLexicalScope`](https://biomejs.dev/linter/rules/use-qwik-valid-lexical-scope) rule to the Qwik domain.

This rule helps you avoid common bugs in Qwik components by checking that your variables and functions are declared in the correct place.

**Invalid:**

```js
// Invalid: state defined outside the component's lexical scope.
let state = useStore({ count: 0 });
const Component = component$(() => {
  return <button onClick$={() => state.count++}>
    Invalid: {state.count}
  </button>;
});
```

**Valid:**

```js
// Valid: state initialised within the component's lexical scope and captured by the event.
const Component = component$(() => {
  const state = useStore({ count: 0 });
  return <button onClick$={() => state.count++}>
    Valid: {state.count}
  </button>;
});
```
