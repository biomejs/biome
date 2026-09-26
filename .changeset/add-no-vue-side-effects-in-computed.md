---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueSideEffectsInComputed`](https://biomejs.dev/linter/rules/no-vue-side-effects-in-computed/), which disallows side effects in Vue computed properties.

A computed value is cached and recomputed only when a dependency changes, so a getter does not run a predictable number of times — and neither does a mutation inside it. The rule reports writes to `this` in an Options API `computed` getter, and writes to setup state in a `computed()` getter: assignments, updates, `delete`, the array methods that reorder or resize in place, and `Object.assign()` into the tracked object. Values created inside the getter are not tracked, so building up a local array or object and returning it is still allowed, and a computed property's setter is not checked.

For example, the following snippet triggers the rule.

```vue
<script>
export default {
  computed: {
    reversed() {
      return this.items.reverse()
    }
  }
}
</script>
```
