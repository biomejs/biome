---
"@biomejs/biome": patch
---

Fixed a bare `>` in the children of an Astro expression being treated as markup, such as `{x && <div>a > b</div>}`.
