---
"@biomejs/biome": patch
---

Fixed [#9585](https://github.com/biomejs/biome/issues/9585): `useArrowFunction` now marks its fix as unsafe because converting a function expression to an arrow function can break code that later calls it with `new`.
