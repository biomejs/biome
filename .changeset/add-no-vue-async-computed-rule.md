---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueAsyncComputed`](https://biomejs.dev/linter/rules/no-vue-async-computed/), which reports asynchronous code in Vue computed properties, such as `async` functions, `await`, promises, `setTimeout()`, and `nextTick()`.

```vue
<script>
export default {
  computed: {
    async user() {
      return fetchUser(this.userId);
    },
  },
};
</script>
```
