---
"@biomejs/biome": patch
---

Fixed an Astro frontmatter block being cut short by a closing tag inside a string or comment.

```astro
---
const a = "</script>";
// </script> in a comment
---
```
