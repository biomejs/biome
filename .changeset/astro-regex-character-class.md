---
"@biomejs/biome": patch
---

Fixed a quote inside a regex character class breaking the rest of an Astro file, such as `const unsafe = /[/"]/;`.
