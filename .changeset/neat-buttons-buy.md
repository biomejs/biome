---
"@biomejs/biome": patch
---

Fixed [#12956](https://github.com/biomejs/biome/issues/12956): Biome now leaves the contents of unsupported `<script>` and `<style>` blocks unchanged instead of trying to format them as JavaScript or CSS. This includes embedded languages such as Less, SCSS, PostCSS, CoffeeScript, and custom `lang` or `type` values.

```astro
<style lang="less">
  @color: red;
  .card { color: @color; }
</style>

<script lang="coffee">
  square = (value) -> value * value
</script>
```

Malformed Astro frontmatter, expressions, scripts, and styles also retain their original text while Biome formats the surrounding markup.
