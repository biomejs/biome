---
"@biomejs/biome": patch
---

Fixed a template literal nested inside `${}` breaking the rest of an Astro file, such as ``const href = `/blog${page === 0 ? '' : `/${page + 1}`}`;``.
