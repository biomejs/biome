---
"@biomejs/biome": patch
---

Fixed [#10212](https://github.com/biomejs/biome/issues/10212): [`useConsistentObjectDefinitions`](https://biomejs.dev/linter/rules/use-consistent-object-definitions/) no longer reports named function expressions such as `{ b: function c() {} }`, because converting them to methods can change the function's `name` or how its name is resolved inside the body.
