---
"@biomejs/biome": patch
---

Fixed [#9768](https://github.com/biomejs/biome/issues/9768): [`noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) no longer reports nested selectors that style different elements. The following code is now valid:

```scss
.aaa {
  ul li {
    &:hover { color: #1684fc; }
  }
}

.bbb {
  a {
    &:hover { background: #fff; }
  }
}
```

The rule now reports `a:hover { color: red; }` followed by `a { color: blue; }`, as Stylelint does, and ignores rules without declarations.
