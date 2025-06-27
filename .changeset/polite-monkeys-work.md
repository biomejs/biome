---
"@biomejs/biome": patch
---

Fixed [#6384](https://github.com/biomejs/biome/issues/6384). The rule [`useAltText`](https://biomejs/dev/linter/rules/no-alt-text) now emits a diagnostic with a correct range, so suppression comments can work correctly.
