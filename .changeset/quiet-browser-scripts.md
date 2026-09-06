---
"@biomejs/biome": patch
---

Added the nursery rule [noTopLevelBrowserGlobals](https://biomejs.dev/linter/rules/no-top-level-browser-globals/), which reports unguarded browser globals at the top level of Vue and Svelte scripts to help prevent server-side rendering errors.

```vue
<script setup>
const width = window.innerWidth;
</script>
```
