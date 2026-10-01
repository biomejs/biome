---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueSideEffectsInComputed`](https://biomejs.dev/linter/rules/no-vue-side-effects-in-computed/), which disallows side effects in Vue computed properties.

For example, the following snippet triggers the rule.

```vue
<script>
export default {
  computed: {
    fullName() {
      this.firstName = 'lorem'
      return `${this.firstName} ${this.lastName}`
    }
  }
}
</script>
```
