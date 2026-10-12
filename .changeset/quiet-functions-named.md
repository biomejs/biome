---
"@biomejs/biome": patch
---

Added the nursery rule [`useNamedFunction`](https://biomejs.dev/linter/rules/use-named-function/), which requires functions written with the `function` keyword to have a name. For example, `Foo.prototype.bar = function () {};` is now reported.
