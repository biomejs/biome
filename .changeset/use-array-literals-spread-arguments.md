---
"@biomejs/biome": patch
---

Fixed [#11942](https://github.com/biomejs/biome/issues/11942): [`useArrayLiterals`](https://biomejs.dev/linter/rules/use-array-literals/) no longer offers a safe fix when the `Array` constructor is called with spread arguments. A spread can expand to a single numeric argument, which the constructor treats as an array length, so the rewrite to an array literal can change the behavior at runtime.
