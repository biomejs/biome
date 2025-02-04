---
"@biomejs/biome": major
---

Changed default formatting of `package.json`.

When Biome encounters a file called `package.json`, by default it will format the file with all objects and arrays expanded.

```diff
- { "name": "project", "dependencies": { "foo": "latest" } }
+ {
+  "projectName": "project",
+  "dependencies": {
+    "foo": "^1.0.0"
+  }
+ }
```
