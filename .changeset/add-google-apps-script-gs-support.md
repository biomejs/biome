---
"@biomejs/biome": minor
---

Added support for Google Apps Script (`.gs`) files. Since Google Apps Script is plain JavaScript running on the V8 runtime (with a shared global scope and no ES module system), `.gs` files are now recognized and processed as JavaScript scripts by the formatter and linter. Closes [#8267](https://github.com/biomejs/biome/issues/8267).
