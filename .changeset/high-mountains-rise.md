---
"@biomejs/biome": patch
---

Fix [#342](https://github.com/biomejs/biome/issues/342), "expected a declaration as guaranteed by is_at_ts_declare_statement" error for declare interface:

```ts
declare interface
```
