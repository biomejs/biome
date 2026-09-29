---
"@biomejs/biome": patch
---

Fixed [#11924](https://github.com/biomejs/biome/issues/11924): [`noFocusedTests`](https://biomejs.dev/linter/rules/no-focused-tests/) no longer reports chained method calls such as `builder.image(url).fit("max")` as focused tests.
