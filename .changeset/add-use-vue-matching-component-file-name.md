---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueMatchingComponentFileName`](https://biomejs.dev/linter/rules/use-vue-matching-component-file-name/), which requires the name of a Vue component to match its file name. The following component in `MyComponent.vue` is now reported:

```vue
<script>
export default {
  name: "MyButton",
};
</script>
```
