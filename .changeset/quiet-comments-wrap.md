---
"@biomejs/biome": patch
---

Fixed the code fixes of [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/), [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/), and [`noVueDataObjectDeclaration`](https://biomejs.dev/linter/rules/no-vue-data-object-declaration/) duplicating comments attached to the wrapped expression. For example, the `noFloatingPromises` fix for an array of promises preceded by a comment no longer copies that comment into `Promise.all(...)`.
