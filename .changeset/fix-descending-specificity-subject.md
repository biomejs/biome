---
"@biomejs/biome": patch
---

Fixed [#9768](https://github.com/biomejs/biome/issues/9768): [`noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) no longer compares nested selectors such as `&:hover` that style different elements. The rule now resolves `&` before comparing, and ignores pseudo-classes and the order of simple selectors when it decides which element a selector styles, so `a:hover { color: red; }` followed by `a { color: blue; }` is now reported, as in Stylelint. Rules without declarations, such as a rule that only contains a comment, are no longer reported.
