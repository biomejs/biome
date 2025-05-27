---
"@biomejs/biome": minor
---

`useNamingConmvention` now ignores unused variables prefixed with an underscore `_`.

This avoids conflicts with the unsafe fix of `noUnusedVariables`.
The following code is now accepted because the variable is unused and prefixed with an underscore.

```js
const _Unknown_Style = 0;
```
