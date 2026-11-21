---
"@biomejs/biome": patch
---

Fixed [#12506](https://github.com/biomejs/biome/issues/12506): [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) now accepts an all-uppercase name such as `URL` where `PascalCase` is expected and `strictCase` is set to `false`.
