---
"@biomejs/biome": patch
---

Fixed [#13029](https://github.com/biomejs/biome/issues/13029): the Markdown formatter now normalizes spaces inside inline and reference link text without removing whitespace-only text.

```diff
-something [ text   ](   destination)
-something [    ](   destination)
+something [ text ](destination)
+something [ ](destination)
```
