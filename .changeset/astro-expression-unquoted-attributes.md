---
"@biomejs/biome": patch
---

Fixed unquoted attribute values being rejected inside an Astro expression, such as `{x && <a class=foo maxlength=255 href=/about>go</a>}`.
