---
"@biomejs/biome": patch
---

Fixed [#9768](https://github.com/biomejs/biome/issues/9768): [`noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) no longer compares nested selectors such as `&:hover` that style different elements. The rule now resolves `&` before comparing, and ignores pseudo-classes when it decides which element a selector styles, so `a:hover {}` followed by `a {}` is now reported, as in Stylelint.
