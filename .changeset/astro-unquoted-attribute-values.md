---
"@biomejs/biome": patch
---

Fixed Astro rejecting HTML5 unquoted attribute values that contain `` ` ``, `=`, `'` or `"`, such as `<a href=a=b>` and `<a href=a'b>`.
