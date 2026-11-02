---
"@biomejs/biome": patch
---

Fixed [#12623](https://github.com/biomejs/biome/issues/12623): the safe fix of [`useConsistentTestIt`](https://biomejs.dev/linter/rules/use-consistent-test-it/) no longer leaves an unused import when applied to every call in a file. When all calls to an imported test function are renamed, the import is now replaced with an alias, such as `test as it`, instead of keeping the original specifier. When the original specifier is kept, the alias is now added as `test, test as it` instead of `test , test as it`.
