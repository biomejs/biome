---
"@biomejs/biome": patch
---

Fixed [#13380](https://github.com/biomejs/biome/issues/13380): [`useNumberNamespace`](https://biomejs.dev/linter/rules/use-number-namespace/) no longer offers a fix when `Number` is shadowed by a local binding, because the fix changed the runtime behavior of the code. The rule still reports the global.
