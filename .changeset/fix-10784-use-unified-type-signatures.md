---
"@biomejs/biome": patch
---

Fixed [#10784](https://github.com/biomejs/biome/issues/10784): [`useUnifiedTypeSignatures`](https://biomejs.dev/linter/rules/use-unified-type-signatures/) no longer offers an unsafe fix to make parameters optional when merging overload signatures of differing arity, preventing breakage of `Parameters<T>` and call-site type-checking under TypeScript. The rule still reports the diagnostic.
