---
"@biomejs/biome": patch
---

Fixed [#11936](https://github.com/biomejs/biome/issues/11936): [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer offers renames for global declarations in TypeScript scripts without imports or exports. The rule still reports naming violations, while preserving names that other files may reference.
