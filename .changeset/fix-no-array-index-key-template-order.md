---
"@biomejs/biome": patch
---

Fixed [#8812](https://github.com/biomejs/biome/issues/8812): [`noArrayIndexKey`](https://biomejs.dev/linter/rules/no-array-index-key/) now correctly detects array index usage in template strings regardless of position. Previously, a key like `` `${index}-${item}` `` was not flagged while `` `${item}-${index}` `` was.
