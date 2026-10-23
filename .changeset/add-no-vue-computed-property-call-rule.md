---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueComputedPropertyCall`](https://biomejs.dev/linter/rules/no-vue-computed-property-call/), which disallows calling a Vue computed property whose value is not a function. Calling such a value throws an error when the code runs.

```vue
<script>
export default {
  computed: {
    total() {
      return 42;
    },
  },
  methods: {
    log() {
      console.log(this.total()); // reported
    },
  },
};
</script>
```
