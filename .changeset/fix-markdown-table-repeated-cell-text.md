---
"@biomejs/biome": patch
---

Fixed [#12542](https://github.com/biomejs/biome/issues/12542): the Markdown formatter no longer idempotent when it formats tables.

```diff
- | a        | b     |
- | -------- | ----- |
- | x        | `y` z |
- | `p` z`q` | r     |
+ | a         | b     |
+ | --------- | ----- |
+ | x         | `y` z |
+ | `p` z `q` | r     |
```
