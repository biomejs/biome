---
"@biomejs/biome": patch
---

Fixed [#9768](https://github.com/biomejs/biome/issues/9768): [`noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) no longer compares nested selectors such as `&:hover` that style different elements. The rule now resolves `&` before comparing, and ignores pseudo-classes and the order of simple selectors when it decides which element a selector styles, so `a:hover { color: red; }` followed by `a { color: blue; }` is now reported, as in Stylelint. Rules without declarations, such as a rule that only contains a comment, are no longer reported. Type selectors are compared case-insensitively, so `A:hover` and `a` style the same element.

[`noDuplicateSelectors`](https://biomejs.dev/linter/rules/no-duplicate-selectors/) now keeps selectors inside functional pseudo-classes such as `:is()` together when it compares selectors, so `.c:is(.a .b) {}` and `:is(.a .b).c {}` are now reported as duplicates.

Both rules now check selectors whose `%` or `#{` is plain text inside a quoted string, such as `[data-progress="50%"]`. Previously, they skipped these selectors as if they contained an SCSS placeholder or interpolation. Selectors with SCSS interpolation, even inside a quoted string such as `[title="#{$x}"]`, are still skipped.
