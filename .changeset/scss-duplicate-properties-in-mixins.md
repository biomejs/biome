---
"@biomejs/biome": patch
---

Fixed [`noDuplicateProperties`](https://biomejs.dev/linter/rules/no-duplicate-properties/) and [`noDuplicateCustomProperties`](https://biomejs.dev/linter/rules/no-duplicate-custom-properties/) missing duplicates in blocks outside a style rule. Both rules now check SCSS `@mixin` bodies and top-level `@include` content blocks.

```scss
@mixin colors {
  color: red;
  color: blue;
}
```

[`noDuplicateProperties`](https://biomejs.dev/linter/rules/no-duplicate-properties/) no longer reports repeated CSS Modules `composes` declarations, which are needed to compose classes from more than one file.
