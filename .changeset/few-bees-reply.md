---
"@biomejs/biome": minor
---

Implemented the `indentScriptAndStyle` option for vue and svelte files, with the default set to `false` to match [Prettier's `vueIndentScriptAndStyle` option](https://prettier.io/docs/options#vue-files-script-and-style-tags-indentation). When enabled, this option indents the content within `<script>` and `<style>` tags to align with the surrounding HTML structure.

It can be enabled with this configuration:

```json
{
  "html": {
    "formatter": {
      "indentScriptAndStyle": true
    }
  }
}
```

Which will format this code to:
```vue
<script>
  import Component from "./Component.vue";
</script>
```
