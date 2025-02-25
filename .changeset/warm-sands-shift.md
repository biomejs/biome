---
"@biomejs/biome": minor
---
[useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/) now provides a code fix.

```diff
- const xs = new Array();
+ const xs = [];
```

The code fix is currently marked as unsafe.
We plan to make it safe in a future release of Biome.
