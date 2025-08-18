---
"@biomejs/biome": patch
---


Added the [`useQwikValidLexicalScope`](https://biomejs.dev/linter/rules/use-qwik-valid-lexical-scope) rule to Qwik.

This rule is intended for use in Qwik applications to validate proper lexical scope usage in Qwik components and prevent common reactivity issues.

**Invalid:**

```js
const Component = component$(() => {
  const state = useStore({ count: 0 }); // Defined in wrong scope

  return <button onClick$={() => state.count++}>
    Invalid: {state.count}
  </button>;
});
```

**Valid:**

```js
const Component = component$(() => {
  return <button onClick$={() => {
    const state = useStore({ count: 0 }); // Correct lexical scope
    state.count++;
  }}>
    Valid: {state.count}
  </button>;
});
```
