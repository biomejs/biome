---
"@biomejs/biome": patch
---

Fixed [#12791](https://github.com/biomejs/biome/issues/12791): [`noDuplicateProperties`](https://biomejs.dev/linter/rules/no-duplicate-properties/) now joins SCSS nested property names the way Sass does, and reports properties that are set twice once the names are joined.

```scss
.a {
  border-radius: 1px;
  border: {
    radius: 2px;
  }
}
```
