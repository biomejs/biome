---
"@biomejs/biome": minor
---

Add new option `javascript.parser.jsxEverywhere`. This new option allows to control whether Biome should expect JSX syntax in `.js`/`.ts` files.

When `jsxEverywhere` is set to `false`, having JSX syntax like `<div></div>` inside `.js`/`.ts` files will result in a **parsing error**.

This option defaults to `true`.
