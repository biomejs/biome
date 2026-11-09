---
"@biomejs/biome": patch
---

Added the nursery rule [`noExcessiveSelectorComplexity`](https://biomejs.dev/linter/rules/no-excessive-selector-complexity/), which limits how many IDs, type selectors, attribute selectors, pseudo-classes, universal selectors, compound selectors, and combinators a CSS selector can contain. It can also disallow chosen combinators, pseudo-classes, pseudo-elements, attributes, and attribute operators. The rule does nothing until an option is set.

For example, with `"maxIds": 1`, the following selector is reported:

```css
#main #sidebar {}
```
