---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueValidDefineOptions`](https://biomejs.dev/linter/rules/use-vue-valid-define-options/), which reports invalid calls to Vue's `defineOptions()` macro in `<script setup>`. It reports a call that is repeated, that has no options or has type arguments, that declares `props`, `emits`, `expose`, or `slots`, or that uses variables declared in `<script setup>`.

```vue
<script setup>
const options = { name: "Foo" };
defineOptions(options);
</script>
```
