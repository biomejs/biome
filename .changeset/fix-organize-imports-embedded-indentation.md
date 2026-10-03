---
"@biomejs/biome": patch
---

Fixed [#9194](https://github.com/biomejs/biome/issues/9194): the [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) assist action could corrupt indentation when sorting imports whose first import had no leading newline, such as the content of a Vue or Svelte `<script>` block formatted with `indentScriptAndStyle: true`. Biome now preserves the indentation shared by the reordered imports instead of dropping or duplicating it.
