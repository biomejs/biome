---
"@biomejs/biome": patch
---

Fixed `noAstroSetHtmlDirective` failing to report `set:html` inside Astro expressions, such as `{<div set:html={content} />}`.
