---
"@biomejs/biome": patch
---

Fix [#6485](https://github.com/biomejs/biome/issues/6485): Handle multiple semicolons correctly in blocks (#6485)

```css
div {
  box-sizing: border-box;;
  color: red;
}
```
