---
"@biomejs/biome": patch
---

Fixed [#12987](https://github.com/biomejs/biome/issues/12987): [`noUnusedTemplateLiteral`](https://biomejs.dev/linter/rules/no-unused-template-literal/) no longer reports template literals that escape `$`, `{` or a backtick, such as `` `tail \${path}` ``. Its fix turned them into strings that `noTemplateCurlyInString` and `noUselessEscapeInString` then reported.
