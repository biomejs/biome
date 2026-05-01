---
"@biomejs/biome": patch
---

Fixed [#10177](https://github.com/biomejs/biome/issues/10177): The HTML parser no longer reports lowercase `html` or `doctype` text as invalid after void elements such as `<br>`.
