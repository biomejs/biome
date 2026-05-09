---
"@biomejs/biome": patch
---

Fixed [#10193](https://github.com/biomejs/biome/issues/10193): `style/useReadonlyClassProperties` no longer reports class properties as readonly-able when they are assigned inside arrow callbacks nested in class property initializers.
