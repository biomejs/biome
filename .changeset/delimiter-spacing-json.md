---
"@biomejs/biome": minor
---

Added `delimiterSpacing` support for JSON. When enabled, Biome inserts spaces inside square brackets when the content fits on a single line. Empty brackets are not affected. It can be configured via `json.formatter.delimiterSpacing`. Defaults to false.

```diff
- [1, 2, 3]
+ [ 1, 2, 3 ]
```
