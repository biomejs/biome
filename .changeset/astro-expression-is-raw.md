---
"@biomejs/biome": patch
---

Fixed `is:raw` children inside an Astro expression being read as JSX, such as `{x && <div is:raw>{not js} < & text</div>}`.
