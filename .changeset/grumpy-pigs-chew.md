---
"@biomejs/biome": minor
---

Added the nursery rule`vue/noVueSetupPropsReactivityLoss`.

This new rule disallows usages that cause the reactivity of `props` passed to the `setup` function to be lost.

Invalid code example:

```jsx
export default {
  setup({ count }) {
    // `count` is no longer reactive here.
    return () => h("div", count);
  },
};
```
