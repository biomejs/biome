---
"@biomejs/biome": patch
---

Fixed [#9541](https://github.com/biomejs/biome/issues/9541): [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/), [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/), and [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) now correctly recognise exported variables and functions declared in one embedded `<script>` block as usable from a sibling `<script>` block, in Svelte's `<script module>`/`<script>` pair and Vue's non-`setup` `<script>` blocks.

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
