---
"@biomejs/biome": patch
---

Fixed an Astro frontmatter block ending early on a line that merely starts with a dash.

```astro
---
--count;
---
```
