---
"@biomejs/biome": patch
---

Fixed [#6695](https://github.com/biomejs/biome/issues/6695): [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) now correctly reports TypeScript parameter properties with modifiers.

Previously, constructor parameter properties with modifiers like `private` or `readonly` were not checked against naming conventions. These properties are now treated consistently with regular class properties.
