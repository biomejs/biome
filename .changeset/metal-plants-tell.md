---
"@biomejs/biome": patch
---

Fixed [#7798](https://github.com/biomejs/biome/issues/7798). [useNamingConvention](https://biomejs.dev/linter/rules/use-naming-convention/) no longer panics when it encounters a name that consists of a single dollar sign `$` that doesn't match a custom convention.
