---
"@biomejs/biome": patch
---

Fixed [#9899](https://github.com/biomejs/biome/issues/9899): the `json` and `json-pretty` reporters now escape backslashes in a diagnostic's `location.path`. Previously, paths containing backslashes (such as Windows-style paths) were emitted unescaped, producing invalid JSON.

```diff
-    "path": "src\account\setup-passkey.tsx",
+    "path": "src\\account\\setup-passkey.tsx",
```
