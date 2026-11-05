---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueRegisteredComponentNameCasing`](https://biomejs.dev/linter/rules/use-vue-registered-component-name-casing/), which requires the names of components registered in the `components` option of a Vue component to use one casing style. By default, names must be in PascalCase. The `casing` option switches the rule to camelCase or kebab-case.

```vue
<script>
export default {
  components: {
    "todo-item": TodoItem,
  },
};
</script>
```
