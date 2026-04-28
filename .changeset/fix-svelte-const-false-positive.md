---
"@biomejs/biome": patch
---

Fixed false positive in `noAssignInExpressions` for Svelte `{@const}` blocks. Assignments in `{@const name = value}` are now correctly recognized as declarations rather than accidental assignments in expressions.
