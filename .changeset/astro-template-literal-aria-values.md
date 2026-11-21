---
"@biomejs/biome": patch
---

Fixed [#12707](https://github.com/biomejs/biome/issues/12707): [`useValidAriaValues`](https://biomejs.dev/linter/rules/use-valid-aria-values/) no longer reports Astro template literal attribute values that contain `${...}` interpolations, because they're evaluated at runtime.

```astro
<button aria-pressed=`${view === "cards" ? "true" : "false"}`>Button</button>
```
