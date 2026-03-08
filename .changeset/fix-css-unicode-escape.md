---
"@biomejs/biome": patch
---

Fixed [`#9385`](https://github.com/biomejs/biome/issues/9385): the [`noUselessEscapeInString`](https://biomejs.dev/linter/rules/no-useless-escape-in-string/) rule no longer strips valid CSS unicode escapes from string literals. This fixes broken iconfont `content` values caused by the autofix.
