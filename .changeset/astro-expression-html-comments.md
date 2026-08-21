---
"@biomejs/biome": patch
---

Fixed HTML comments inside an Astro expression failing to parse. They are now read as trivia, wherever they appear among the children.

```astro
{x && <div><!-- first -->text<!-- last --></div>}
{cond && <a></a><!-- c --><b></b>}
```
