---
"@biomejs/biome": patch
---


Added the [`useQwikMethodUsage`](https://biomejs.dev/linter/rules/use-qwik-method-usage) lint rule for the Qwik domain.

This rule validates Qwik hook usage. Identifiers matching `useXxx` must be called only within serialisable reactive contexts (for example, inside `component$`, route loaders/actions, or within other Qwik hooks), preventing common Qwik antipatterns.

**Invalid:**

```js
// Top-level hook call is invalid.
const state = useStore({ count: 0 });

function helper() {
  // Calling a hook in a non-reactive function is invalid.
  const loc = useLocation();
}
```

**Valid:**

```js
component$(() => {
  const state = useStore({ count: 0 }); // OK inside component$.
  return <div>{state.count}</div>;
});

const handler = $(() => {
  const loc = useLocation(); // OK inside a $-wrapped closure.
  console.log(loc.params);
});
```
