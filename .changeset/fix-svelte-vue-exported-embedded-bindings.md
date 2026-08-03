---
"@biomejs/biome": patch
---

Fixed [#9541](https://github.com/biomejs/biome/issues/9541), where `noUndeclaredVariables` (and other rules relying on cross-script-block bindings, such as `noUnusedImports` and `noUnusedVariables`) failed to recognize a top-level binding declared with `export` (e.g. `export const foo = ...`, `export function foo() {}`) inside one embedded script block as visible from a sibling script block. This affected Svelte's `<script module>`/`<script>` pair and Vue's non-`setup` `<script>` blocks. Non-exported bindings were already handled correctly; only the `export`-prefixed declaration form was missed.
