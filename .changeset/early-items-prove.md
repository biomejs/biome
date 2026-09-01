---
"@biomejs/biome": patch
---

Fixed [#5091](https://github.com/biomejs/biome/issues/5091): Biome no longer moves comments next to the `<` of a generic, which causes invalid TypeScript syntax:

```diff
- Generic<// a comment
+ Generic<
+   // a comment
```
