---
"@biomejs/biome": patch
---

Fixed a bare `<` in Astro text being treated as the start of a tag, such as `<p>5 < 6 and 7 > 6</p>`. As in HTML, a `<` that cannot open a tag is text and needs no escaping.
