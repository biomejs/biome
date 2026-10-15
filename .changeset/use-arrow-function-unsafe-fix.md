---
"@biomejs/biome": patch
---

Fixed [#12372](https://github.com/biomejs/biome/issues/12372): the code action of [`useArrowFunction`](https://biomejs.dev/linter/rules/use-arrow-function/) is now unsafe. A `function` expression has a `prototype` property and an arrow function does not, so a safe fix would change program behavior.
