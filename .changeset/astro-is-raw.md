---
"@biomejs/biome": patch
---

Fixed the children of an Astro element carrying `is:raw` being parsed as markup instead of raw text.

```astro
<article is:raw><% awesome %></article>
<div is:raw><p>{x}</p></div>
```
