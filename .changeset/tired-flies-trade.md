---
"@biomejs/biome": patch
---

Fixed [#7527](https://github.com/biomejs/biome/issues/7527): suppression actions for diagnostics emitted on comments are now inserted before the diagnostic comment. In particular, suppressing [`noTsIgnore`](https://biomejs.dev/linter/rules/no-ts-ignore/) now places the `biome-ignore` comment before `@ts-ignore`.
