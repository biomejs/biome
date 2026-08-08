---
"@biomejs/biome": patch
---

Fixed [#9541](https://github.com/biomejs/biome/issues/9541). Biome now correctly recognises exported variables and functions declared in one embedded `<script>` block as usable from a sibling `<script>` block. This affects [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/), [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/), and [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/), for Svelte's `<script module>`/`<script>` pair and Vue's non-`setup` `<script>` blocks.

For example, Biome no longer reports `greet` as undeclared in the following Svelte component:

```svelte
<script module>
  export function greet() {
    console.log("Hello!");
  }
</script>

<script>
  greet();
</script>
```
