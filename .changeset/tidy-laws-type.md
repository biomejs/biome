---
"@biomejs/biome": patch
---

Fixed [#11946](https://github.com/biomejs/biome/issues/11946): [`noUnreachable`](https://biomejs.dev/linter/rules/no-unreachable/) and [`useGetterReturn`](https://biomejs.dev/linter/rules/use-getter-return/) now recognize `while` and `do...while` loops with truthy literal conditions as infinite unless control flow exits the loop.
