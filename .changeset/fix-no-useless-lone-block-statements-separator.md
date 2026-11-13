---
"@biomejs/biome": patch
---

Fixed [#13127](https://github.com/biomejs/biome/issues/13127): the safe fix of [`noUselessLoneBlockStatements`](https://biomejs.dev/linter/rules/no-useless-lone-block-statements/) no longer produces invalid code when the removed block was the only thing separating two statements. Biome now adds a semicolon to the statement that would otherwise be merged with the next one.

```js
// Before
{ a() } b()
// After the fix
a(); b()
```
