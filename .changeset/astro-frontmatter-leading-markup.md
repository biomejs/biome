---
"@biomejs/biome": patch
---

Fixed `---` being read as an Astro frontmatter fence when markup precedes it. Astro only recognizes frontmatter at the very start of a file, so a file opening with a comment now has no frontmatter, and its `---` lines are content.

```astro
<!-- c -->
---
this is text, not frontmatter
---
```
