---
"@biomejs/biome": patch
---

Update documentation & rule source for [`lint/complexity/noBannedTypes`](https://biomejs.dev/linter/rules/no-banned-types).

Among other things, the rule now recommends `Record<keyof any, never>` instead of `Record<string, never>`