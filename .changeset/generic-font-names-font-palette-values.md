---
"@biomejs/biome": patch
---

Fixed [`useGenericFontNames`](https://biomejs.dev/linter/rules/use-generic-font-names/) reporting `font-family` inside `@font-palette-values`. There, `font-family` names the font families the palette applies to, so it doesn't take a generic family.

```css
@font-palette-values --palette {
  font-family: Bixa;
  override-colors: 0 red;
}
```
