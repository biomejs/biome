---
"@biomejs/biome": patch
---

Fixed a false negative in [noUnreachable](https://biomejs.dev/linter/rules/no-unreachable/) and [useGetterReturn](https://biomejs.dev/linter/rules/use-getter-return/): `for` loops whose test is a truthy literal, such as `for (; true ;)`, are now handled as infinite loops, consistent with `while` and `do...while`.
