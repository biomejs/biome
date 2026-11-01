---
"@biomejs/biome": patch
---

Added the new nursery rule [`noVueThisInBeforeRouteEnter`](https://biomejs.dev/linter/rules/no-vue-this-in-before-route-enter/), which reports `this` in the `beforeRouteEnter` navigation guard of a Vue component. Vue Router calls this guard before it creates the component, so `this` is `undefined` there.

```vue
<script>
export default {
  beforeRouteEnter(to, from, next) {
    this.loadUser(to.params.id); // reported
    next();
  },
};
</script>
```
