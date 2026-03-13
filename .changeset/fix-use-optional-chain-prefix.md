---
"@biomejs/biome": patch
---

Fixed [#7214](https://github.com/biomejs/biome/issues/7214): [`useOptionalChain`](https://biomejs.dev/linter/rules/use-optional-chain/) now detects optional chain patterns that don't start at the beginning of a logical AND expression. For example, `bar && foo && foo.length` is now correctly flagged and fixed to `bar && foo?.length`.
