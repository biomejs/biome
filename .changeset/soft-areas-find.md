---
"@biomejs/biome": patch
---

Fixed [#13028](https://github.com/biomejs/biome/issues/13028): Biome now keeps a paragraph in its outer block quote when a quote-only blank line closes a nested block quote.

```diff
 > > b
 >
-> > c
+> c
```
