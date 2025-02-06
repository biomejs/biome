---
"@biomejs/biome": patch
---

[useArrowFunction](https://biomejs.dev/linter/rules/use-arrow-function/) now preserves directives ([#4530](https://github.com/biomejs/biome/issues/4530)).

Previously the rule removed the directives when a function expression was turned into an arrow function.
The rule now correctly keeps the directives.

```diff
- const withDirective = function () {
+ const withDirective = () => {
    "use server";
    return 0;
  }
```
