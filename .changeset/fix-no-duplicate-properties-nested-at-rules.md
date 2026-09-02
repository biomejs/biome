---
"@biomejs/biome": patch
---

Fixed [#6615](https://github.com/biomejs/biome/issues/6615). [`noDuplicateProperties`](https://biomejs.dev/linter/rules/no-duplicate-properties/) no longer reports declarations nested in block at-rules as duplicates of declarations in their parent block.
