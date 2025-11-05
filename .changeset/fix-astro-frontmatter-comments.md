---
"@biomejs/biome": patch
---

Fixed a regression in Astro frontmatter parsing where comments inside quoted strings were incorrectly detected as actual comments. This caused the parser to prematurely terminate frontmatter parsing when encountering strings like `const test = "//";`.
For example, the following Astro frontmatter now parses correctly:

```astro
---
const test = "// not a real comment";
---
```
