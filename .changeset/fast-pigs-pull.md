---
"@biomejs/biome": patch
---

Implemented the runtime default-export case from [#11463](https://github.com/biomejs/biome/issues/11463) with the new [`noAstroComponentDefaultExport`](https://biomejs.dev/linter/rules/no-astro-component-default-export/) nursery rule. The rule reports runtime default exports in Astro component frontmatter while allowing named and type-only exports.

```astro
---
export default function render() {}
---
```
