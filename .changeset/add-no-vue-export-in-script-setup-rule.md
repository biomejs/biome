---
"@biomejs/biome": patch
---

Added the nursery rule [`noVueExportInScriptSetup`](https://biomejs.dev/linter/rules/no-vue-export-in-script-setup/), which disallows `export` in Vue `<script setup>` blocks. Vue fails to compile a component when its `<script setup>` block exports a value. TypeScript type exports are allowed.

```vue
<script setup>
export const message = "Hello";
</script>
```
