---
"@biomejs/biome": patch
---

Fixed expressions inside an Astro `<pre>` or `<textarea>` being read as raw text. Astro parses both as ordinary elements, so their markup and interpolations are now parsed, and a variable used only inside one is no longer reported as unused.

```astro
<pre>{value}</pre>
<textarea><div>{value}</div></textarea>
```
