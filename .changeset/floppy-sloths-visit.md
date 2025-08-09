---
"@biomejs/biome": minor
---

Added the new rule `useBiomeIgnoreFolder`. Since v2.2, Biome correctly prevents the indexing and crawling of folders.

However, the correct pattern has changed. This rule attempts to detect incorrect usage, and promote the new pattern:

```diff
// biome.json
{
  "files": {
    "includes": [
-      "!dist/**",
-      "!**/fixtures/**",
+      "!dist",
+      "!**/fixtures",
    ]
  }
}
```
