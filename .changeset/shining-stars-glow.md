---
"@biomejs/biome": patch
---

Fixed [#4533](https://github.com/biomejs/biome/issues/4533): `noUnknownPseudoClass` no longer reports pseudo classes after a webkit scrollbar pseudo element.

The following code will no longer report a diagnostic:

```css
::-webkit-scrollbar-thumb:hover {}
```
