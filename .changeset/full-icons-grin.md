---
"@biomejs/biome": patch
---

Fixed [#11528](https://github.com/biomejs/biome/issues/11528): [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) no longer reports statement-level `await` expressions that handle Promise values, including overloaded calls returning Promise aliases. Awaited values that resolve to arrays of Promises remain reported because their element Promises are not handled by `await`.
