---
"@biomejs/biome": patch
---


Added the [`useQwikMethodUsage`](https://biomejs.dev/linter/rules/use-qwik-method-usage) rule to Qwik.

This rule ensures proper method declaration patterns in Qwik applications to maintain serializability and prevent common Qwik anti-patterns.

**Invalid:**

```js
class Counter {
  increment() {  // Class methods are invalid in Qwik components
    this.count++;
  }
}
```

**Valid:**

```js
const increment = $(() => {  // Proper Qwik method declaration
  const state = useStore({ count: 0 });
  state.count++;
});
```
