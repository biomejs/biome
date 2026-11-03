---
"@biomejs/biome": patch
---

Added the nursery rule [`useVueDefaultExport`](https://biomejs.dev/linter/rules/use-vue-default-export/), which reports a `<script>` block of a Vue component that has no default export, when the file has no `<script setup>` block.

```vue
<script>
const foo = "foo";
</script>
```
