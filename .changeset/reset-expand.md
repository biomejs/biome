---
"@biomejs/biome": patch
---

Fixed the formatter sometimes keeping an enclosing group expanded after it formatted
embedded content, which produced unnecessary line breaks. For example, an embedded
expression in an Astro attribute no longer forces the element onto several lines:

```diff
-<div
-	class={[].map((item: string) => item).join()}
-></div>
+<div class={[].map((item: string) => item).join()}></div>
```
