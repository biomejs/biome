---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueComputedPropertiesInData`](https://biomejs.dev/linter/rules/no-vue-computed-properties-in-data/), which reports computed properties used inside the `data` function of a Vue component. Vue runs `data` before it sets up computed properties, so the value read there is `undefined`.

```vue
<script>
export default {
  data() {
    return { total: this.doubled };
  },
  computed: {
    doubled() {
      return 2;
    },
  },
};
</script>
```
