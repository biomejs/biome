---
"@biomejs/biome": patch
---

Fixed a regression in Astro frontmatter parsing where comments inside quoted strings were incorrectly detected as actual comments. This caused the parser to prematurely terminate frontmatter parsing when encountering strings like `const test = "//";`.

The issue was in the `QuotesSeen` logic which was detecting comment markers (`//` and `/*`) before checking if they were inside quotes. The fix ensures that comment detection only occurs when not inside any quoted strings.

For example, the following Astro frontmatter now parses correctly:

```astro
---
const test = "// not a real comment";
---
```
