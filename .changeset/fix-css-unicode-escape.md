---
"@biomejs/biome": patch
---

Fixed [#9385](https://github.com/biomejs/biome/issues/9385): the [`noUselessEscapeInString`](https://biomejs.dev/linter/rules/no-useless-escape-in-string/) rule no longer removes valid CSS unicode escape sequences (e.g. `\e7bb`, `\e644`) from string literals. Previously, backslashes followed by hex digits `8`–`9` and `a`–`f`/`A`–`F` were incorrectly treated as useless escapes, breaking iconfont `content` properties.
