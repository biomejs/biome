---
"@biomejs/biome": patch
---

Fix [#3836](https://github.com/biomejs/biome/issues/3836), css parser allow multiple semicolons after a declaration, the following example will now parsed correctly:

```css
.foo {
  color: red;;
}
```
