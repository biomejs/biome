---
"@biomejs/biome": patch
---

Fixed the Astro parser failing to recover from a malformed closing tag such as `<div></{<//`, so that a later mistake is reported where it happens rather than cascading.
