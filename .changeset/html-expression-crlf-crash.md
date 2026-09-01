---
"@biomejs/biome": patch
---

Fixed the formatter crashing on an Astro or Svelte expression spanning several lines in a file with CRLF line endings, such as `<p>{a +\r\n  b}</p>`.
