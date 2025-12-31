---
"@biomejs/biome": patch
---

Added the nursery lint rule [`noExcessiveLinesPerFile`](https://biomejs.dev/linter/rules/no-excessive-lines-per-file/).
Biome now reports files that exceed a configurable line limit.

```js
// maxLines: 2
const a = 1;
const b = 2;
const c = 3;
```
