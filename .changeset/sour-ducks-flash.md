---
"@biomejs/biome": patch
---

Fixed an issue where the HTML formatter mangled the closing `>` of inline elements when followed by self-closing elements like `<br>` or `<img>`.
