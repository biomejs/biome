---
"@biomejs/biome": patch
---

Fixed [#13024](https://github.com/biomejs/biome/issues/13024): the formatter no longer breaks the line right after `@return`, `@debug`, `@warn` and `@error` when their comma-separated value doesn't fit. The first item stays on the keyword's line and the remaining items fill the indented lines below, as Prettier does for `@return` and `@debug`.

```diff
- @return
- 	aaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbbbbb,
- 	cccccccccccccccccccccccc dddddddddddddddddddddd;
+ @return aaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbbbbb,
+ 	cccccccccccccccccccccccc dddddddddddddddddddddd;
```
