---
"@biomejs/biome": minor
---

Added new option `javascript.parser.jsxEverywhere`. This new option allows to control whether Biome should expect JSX syntax in `.js`/`.mjs`/`.cjs` files.

When `jsxEverywhere` is set to `false`, having JSX syntax like `<div></div>` inside `.js`/`.mjs`/`.cjs` files will result in a **parsing error**.

Despite the name of the option, JSX is never supported inside `.ts` files. This is because TypeScript generics syntax may conflict with JSX in such files.

This option defaults to `true`.
