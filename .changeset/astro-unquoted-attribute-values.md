---
"@biomejs/biome": patch
---

Fixed Astro rejecting HTML5 unquoted attribute values that contain `` ` ``, `=`, `'` or `"`.

```astro
<a href=a=b>equals</a>
<a href=a'b>squote</a>
```
