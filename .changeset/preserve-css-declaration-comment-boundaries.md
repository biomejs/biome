---
"@biomejs/biome": patch
---

Fixed CSS and SCSS formatting for comments around declaration colons so comments between property names, colons, and values stay at the same boundary as Prettier.

Previously, Biome could move these comments away from the colon boundary:

```css
.selector {
  color: /* red, */
    blue;
}
```

Biome now preserves the comment placement:

```css
.selector {
  color: /* red, */ blue;
}
```
