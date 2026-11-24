---
"@biomejs/biome": patch
---

Fixed [`noUnusedClasses`](https://biomejs.dev/linter/rules/no-unused-classes/) reporting classes of SCSS partials loaded with `@use`, `@forward` or `@import`. Biome now resolves these loads the way Sass does, including partials such as `_card.scss`, `index` files, and import-only files, so a class in `_card.scss` counts as used when a component imports `styles.scss`, which contains `@use "card"`.
