---
"@biomejs/biome": patch
---

Fixed [`noUnusedClasses`](https://biomejs.dev/linter/rules/no-unused-classes/) reporting SCSS classes that are only used through `@extend`. A class that the same file extends now counts as used, and the class in the `@extend` itself is no longer reported.

```scss
.base {
  color: red;
}

.button {
  @extend .base;
}
```
