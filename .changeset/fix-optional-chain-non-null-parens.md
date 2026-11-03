---
"@biomejs/biome": patch
---

pr: #12745

Fixed the formatter removing the parentheses that end an optional chain when a non-null assertion, type arguments or more parentheses follow them. The output either short-circuited the rest of the chain, as in `a?.b!.c` for `(a?.b)!.c`, or didn't parse, as with `new a?.b!()`, a template tag or an assignment target.

```diff
- a?.b!.c;
+ (a?.b)!.c;
- new a?.b!();
+ new (a?.b)!();
- a?.b`template`;
+ (a?.b)`template`;
```
