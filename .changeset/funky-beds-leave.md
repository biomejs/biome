---
"@biomejs/biome": patch
---

Fixed [#12707](https://github.com/biomejs/biome/issues/12707): [`useValidAriaValues`](https://biomejs.dev/linter/rules/use-valid-aria-values/) no longer reports Astro template-literal attributes that contain substitutions because their runtime value is unknown. Static template-literal attributes remain checked.
