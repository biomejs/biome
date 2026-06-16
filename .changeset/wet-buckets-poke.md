---
"@biomejs/biome": patch
---

Fixed [#10637](https://github.com/biomejs/biome/issues/10637): [`useValidAriaValues`](https://biomejs.dev/linter/rules/use-valid-aria-values/) no longer reports a false positive for dynamic ARIA bindings in Svelte and Astro, such as `aria-rowcount={count}`, `aria-rowcount="{count}"`, and `aria-rowcount="row-{count}"`.
