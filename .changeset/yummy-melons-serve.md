---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueSetupPropsReactivityLoss`](https://biomejs.dev/linter/rules/no-vue-setup-props-reactivity-loss/).

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
