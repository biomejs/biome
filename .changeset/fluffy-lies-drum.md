---
"@biomejs/biome": patch
---

Fixed [#7160](https://github.com/biomejs/biome/issues/7160). Now Biome correctly computes ignored files when using `formatter.includes`, `linter.includes` and `assist.includes` inside nested configurations that use `"extends": "//"`.
