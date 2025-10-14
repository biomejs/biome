---
"@biomejs/biome": minor
---

Enhanced the `init` command. The `init` command now checks if the existing project contains known ignore files and known generated folders.

If Biome finds `.gitignore` or `.ignore` files, it will add the following configuration to `biome.json`:
```diff
{
+  "vcs": {
+    "enabled": true,
+    "clientKind": "git",
+    "useIgnoreFile": true
+  }
}
```

If Biome finds a `dist/` folder, it will exclude it automatically using the double-exclude syntax:

```diff
{
+  "files": {
+    "includes": ["**", "!!**/dist"]
+  }
}
```
