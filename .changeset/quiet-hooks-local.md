---
"@biomejs/biome": patch
---

Fixed [#12231](https://github.com/biomejs/biome/issues/12231): [`noDuplicateTestHooks`](https://biomejs.dev/linter/rules/no-duplicate-test-hooks/) no longer reports repeated calls to a local function or variable that is named like a test hook, such as `before`.
