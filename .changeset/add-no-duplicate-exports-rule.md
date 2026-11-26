---
"@biomejs/biome": patch
---

Added the nursery rule [`noDuplicateExports`](https://biomejs.dev/linter/rules/no-duplicate-exports/). It reports a module or TypeScript namespace that exports the same name more than once, or has more than one default export. It also reports two `export * from` statements that export the same name from different declarations. The rule belongs to the `project` domain.

```js
export const foo = 1;
function bar() {}
export { bar as foo };
```
