---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueValidDefaultProp`](https://biomejs.dev/linter/rules/use-vue-valid-default-prop/), which requires the default value of a Vue prop to match the type of the prop. Default objects and arrays must also be returned from a function, so that each component instance gets its own copy.

```vue
<script>
export default {
  props: {
    count: { type: Number, default: "zero" }, // reported
    items: { type: Array, default: [] }, // reported
  },
};
</script>
```
