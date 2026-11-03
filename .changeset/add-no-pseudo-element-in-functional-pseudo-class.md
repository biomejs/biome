---
"@biomejs/biome": patch
---

Added the nursery rule [`noPseudoElementInFunctionalPseudoClass`](https://biomejs.dev/linter/rules/no-pseudo-element-in-functional-pseudo-class/), which disallows pseudo-elements inside `:is()`, `:where()`, `:not()` and `:has()`. Browsers drop these selectors, so the styles never apply to the pseudo-element.

```css
.card:is(::before, ::after) {
  content: "";
}
```
