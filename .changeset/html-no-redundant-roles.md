---
"@biomejs/biome": minor
---

Added the HTML lint rule [`noRedundantRoles`](https://biomejs.dev/linter/rules/no-redundant-roles/). This rule enforces that explicit `role` attributes are not the same as the implicit/default role of an HTML element. It supports HTML, Vue, Svelte, and Astro files.

```html
<!-- Invalid: role="button" is redundant on <button> -->
<button role="button"></button>
```
