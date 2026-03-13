---
"@biomejs/biome": patch
---

Fixed [#6921](https://github.com/biomejs/biome/issues/6921): `noShadow` no longer incorrectly flags destructured variable bindings in sibling scopes as shadowing. Object destructuring, array destructuring, nested patterns, and rest elements are now properly recognized as declarations.
