---
"@biomejs/biome": patch
---

Fixed [#8434](https://github.com/biomejs/biome/issues/8434): [`noBannedTypes`](https://biomejs.dev/linter/rules/no-banned-types/) no longer reports the empty object type `{}` when it is used in a type-parameter constraint union that expresses a non-nullish type, such as `<T extends {} | null>` or `<T extends {} | undefined>`. These are common idioms for "any value except `null`/`undefined`", just like the already-allowed `<T extends {}>`. `{} | null | undefined` is still reported because it is equivalent to `unknown`.
