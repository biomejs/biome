---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVuePropTypeConstructor`](https://biomejs.dev/linter/rules/use-vue-prop-type-constructor/), which reports Vue prop types that aren't constructors, such as a type name written as a string.

```vue
<script>
export default {
  props: {
    size: 'Number', // reported, use `Number` instead
  },
};
</script>
```
