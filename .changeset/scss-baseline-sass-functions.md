---
"@biomejs/biome": patch
---

Fixed [`useBaseline`](https://biomejs.dev/linter/rules/use-baseline/) reporting CSS math functions such as `round()` and `abs()` in SCSS when Sass evaluates them at compile time. In SCSS, the rule now reports these functions only when Sass emits them as written: an argument is interpolated, calls `var()` or a CSS function such as `env()`, or uses units Sass can't convert into each other, such as `%` and `px`.

```scss
$offset: -2px;

.a {
  width: round(1.6px, 1px);
  height: abs($offset);
}
```
