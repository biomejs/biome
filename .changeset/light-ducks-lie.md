---
"@biomejs/biome": patch
---

Added a new nursery rule `noAstroSetHtmlDirective`, which disallows Astro's `set:html` directive because untrusted content can introduce cross-site scripting vulnerabilities.

For example, the following snippet triggers the rule:

```astro
<div set:html={content} />
```
