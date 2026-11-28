---
"@biomejs/biome": patch
---

Fixed [#5172](https://github.com/biomejs/biome/issues/5172): the JUnit reporter now includes formatting diagnostics, so `biome check` and `biome format` list the affected files. Test cases are now grouped into one `<testsuite>` per file.
