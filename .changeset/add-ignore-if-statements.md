---
"@biomejs/biome": patch
---

Added the option `ignoreIfStatements` to [useNullishCoalescing](https://biomejs.dev/linter/rules/use-nullish-coalescing/). Biome now flags `if` statements that only assign to a nullish variable (such as `if (!a) { a = b }`) and can rewrite them to `??=`. When enabled, Biome ignores those `if` statements.
