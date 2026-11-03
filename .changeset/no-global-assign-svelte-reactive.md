---
"@biomejs/biome": patch
---

Fixed [#12687](https://github.com/biomejs/biome/issues/12687): [`noGlobalAssign`](https://biomejs.dev/linter/rules/no-global-assign/) no longer reports Svelte reactive declarations such as `$: length = 2` that are named like a global. In a Svelte component, a top-level `$:` assignment declares a component variable instead of assigning to the global. Assignments in a reactive block, as in `$: { length = 2 }`, are still reported.
