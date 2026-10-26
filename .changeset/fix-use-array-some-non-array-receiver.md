---
"@biomejs/biome": patch
---

Fixed [#12630](https://github.com/biomejs/biome/issues/12630): [`useArraySome`](https://biomejs.dev/linter/rules/use-array-some/) no longer reports calls on receivers that are known not to be arrays, tuples, typed arrays, or iterators, or calls with non-callable arguments.
