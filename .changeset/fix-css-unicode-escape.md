---
"@biomejs/biome": patch
---

Fixed [#9385](https://github.com/biomejs/biome/issues/9385): the CSS [`noUselessEscapeInString`](https://biomejs.dev/linter/rules/no-useless-escape-in-string/) rule now correctly recognizes CSS unicode escapes (`\` followed by 1-6 hex digits). Previously, the rule used JavaScript escape rules and only treated `\0`-`\7` as meaningful, causing valid CSS hex escapes like `\e7bb` or `\e644` (used in icon fonts) to be incorrectly flagged and removed by the auto-fix.
