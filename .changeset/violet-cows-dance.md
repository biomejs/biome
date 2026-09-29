---
"@biomejs/biome": patch
---

Fixed [#11900](https://github.com/biomejs/biome/issues/11900): [`useForOf`](https://biomejs.dev/linter/rules/use-for-of/) no longer reports loops that update their index inside the body, including `i += 1` and `argv[++i]`.
