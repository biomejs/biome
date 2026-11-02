---
"@biomejs/biome": minor
---

Added the `css.formatter.expand` option, which also follows the global `formatter.expand`. With `never`, the SCSS formatter keeps maps, and lists used as map values, on one line when they fit, instead of always expanding them. `auto` and `always` keep the current, Prettier-compatible layout.

```diff
-$inks: (
-	"nav": (
-		--be-nav,
-		rgb(0, 0, 229),
-	),
-);
+$inks: ("nav": (--be-nav, rgb(0, 0, 229)));
```

When migrating a Prettier configuration with `objectWrap: "collapse"`, `biome migrate prettier` now also sets `css.formatter.expand` to `auto`, because Prettier's `objectWrap` doesn't affect SCSS maps.
