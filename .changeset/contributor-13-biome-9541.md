---
"@biomejs/biome": patch
---

Fixed [#9541](https://github.com/biomejs/biome/issues/9541): Svelte `noUndeclaredVariables` analysis now keeps exported declarations from `<script module>` blocks visible to the rest of the file.

Previously, bindings declared as `export const`, `export function`, and similar module-script declarations were dropped from embedded binding collection, so later instance scripts could report false undeclared-variable diagnostics.
