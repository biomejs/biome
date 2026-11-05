---
"@biomejs/biome": patch
---

Fixed [#12827](https://github.com/biomejs/biome/issues/12827): [`noDuplicateProperties`](https://biomejs.dev/linter/rules/no-duplicate-properties/), [`noDuplicateCustomProperties`](https://biomejs.dev/linter/rules/no-duplicate-custom-properties/), and [`noMissingVarFunction`](https://biomejs.dev/linter/rules/no-missing-var-function/) now check declarations at the top level of embedded CSS templates, such as `` styled.div`...` ``.
