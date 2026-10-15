---
"@biomejs/biome": minor
---

Added `consistent` to `javascript.formatter.quoteProperties`. When set, Biome quotes every property name of an object, class, interface, or type literal if at least one of its names needs quotes, matching Prettier.

```diff
-const myObject = { "prop-1": 1, prop2: 2 };
+const myObject = { "prop-1": 1, "prop2": 2 };
```
