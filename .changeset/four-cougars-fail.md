---
"@biomejs/biome": patch
---

Fixed [#8967](https://github.com/biomejs/biome/issues/8967). [useExhaustiveDependencies](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) no longer reports false positives for variables destructured from a rest pattern.
