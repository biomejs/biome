---
"@biomejs/biome": patch
---

Fixed the formatter removing the parentheses around an object, function or class expression at the start of a statement, and around an object at the start of an arrow function body, when one of its members is updated with `++` or `--` or assigned through a non-null assertion. This produced code that doesn't parse.

```diff
- {}.count++;
+ ({}).count++;
- const increment = () => {}.count++;
+ const increment = () => ({}).count++;
```
