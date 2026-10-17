---
"@biomejs/biome": patch
---

Fixed [#12231](https://github.com/biomejs/biome/issues/12231): [`noDuplicateTestHooks`](https://biomejs.dev/linter/rules/no-duplicate-test-hooks/) no longer reports repeated calls to uniquely declared, unreassigned local functions named like test hooks. This covers function declarations and variables initialized with function expressions; other bindings retain their existing duplicate checks.


