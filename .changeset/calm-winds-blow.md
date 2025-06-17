---
"@biomejs/biome": patch
---

Fixed [#3836](https://github.com/biomejs/biome/issues/3836): The CSS parser will now correctly parse the following:

```css
.foo {
  color: red;;
}
```
