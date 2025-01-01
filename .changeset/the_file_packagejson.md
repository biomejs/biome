---
cli: major
---

# Changed default formatting of `package.json`

When Biome encounters a file called `package.json`, by default it will format the file with all lists expanded.

```diff
- { "name": "project", "dependencies": { "foo": "latest" } }
+ {
+  "projectName": "project",
+  "dependencies": {
+    "foo": "^1.0.0"
+  }
+ }
```
