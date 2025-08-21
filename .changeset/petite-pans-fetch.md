---
"@biomejs/biome": patch
---

Resolved an overcorrection in [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) when importing explicit index files.

Imports that explicitly reference an index file are now preserved and no longer rewritten to nested index paths.

#### Example

```diff
// Before
-      import "./sub/index";
+      import "./sub/index/index.js";

// After
-      import "./sub/index";
+      import "./sub/index.js";
```
