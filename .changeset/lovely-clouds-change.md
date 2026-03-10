---
"@biomejs/biome": patch
---

Fixed [#9433](https://github.com/biomejs/biome/issues/9433): [`noBlankTarget`](https://biomejs.dev/linter/rules/no-blank-target/) now correctly handles dynamic href attributes, such as `<a href={company?.website} target="_blank">`.
