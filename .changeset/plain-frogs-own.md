---
"@biomejs/biome": patch
---

Added seven experimental Astro lint rules: [`noAstroExportsFromComponents`](https://biomejs.dev/linter/rules/no-astro-exports-from-components/), [`noAstroUnsafeInlineScripts`](https://biomejs.dev/linter/rules/no-astro-unsafe-inline-scripts/), [`noAstroUnusedCssSelector`](https://biomejs.dev/linter/rules/no-astro-unused-css-selector/), [`noAstroUnusedDefineVarsInStyle`](https://biomejs.dev/linter/rules/no-astro-unused-define-vars-in-style/), [`useAstroClassListDirective`](https://biomejs.dev/linter/rules/use-astro-class-list-directive/), [`useAstroObjectClassList`](https://biomejs.dev/linter/rules/use-astro-object-class-list/), and [`useAstroSplitClassList`](https://biomejs.dev/linter/rules/use-astro-split-class-list/).

For example, the rules report the relevant declarations and attributes in this component:

```astro
---
export default {};
const color = "red";
const active = true;
---
<script>console.log("inline");</script>
<div class={active ? "active" : ""}></div>
<div class:list={active ? "active" : ""}></div>
<div class:list={"card featured"}></div>
<style define:vars={{ color, unused: "blue" }}>
.unused { color: var(--color); }
</style>
```
