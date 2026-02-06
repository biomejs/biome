---
"@biomejs/biome": patch
---

Fixed [#8645](https://github.com/biomejs/biome/issues/8645).
[useAwait](https://biomejs.dev/linter/rules/use-await/) no longer reports `async` generator functions that use `yield*`, since `yield*` in an async generator delegates to an `AsyncIterable` and requires the `async` modifier.
