---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueSlotsAsFunctions`](https://biomejs.dev/linter/rules/use-vue-slots-as-functions/), which reports slots in `this.$slots` that are used as their content instead of being called. In Vue 3, each slot is a function that returns its content.

```js
export default {
  render() {
    return this.$slots.default.filter(isVisible); // reported
  },
};
```
