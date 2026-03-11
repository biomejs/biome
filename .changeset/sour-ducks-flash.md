---
"@biomejs/biome": patch
---

Fixed [#9310](https://github.com/biomejs/biome/issues/9310). Now the HTML formatter doesn't mangle elements that are followed by self-closing elements such as  `<br>` or `<img>`.
