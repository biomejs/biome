---
"@biomejs/biome": patch
---

Fixed [#13384](https://github.com/biomejs/biome/issues/13384): standalone `.svelte.js` and `.svelte.ts` modules no longer treat `$store` as an implicit subscription to `store`. Svelte lint rules now distinguish calls to parameters named `$state`, `$inspect`, or `$derived` from calls to the corresponding runes. The [`noSvelteUselessDerivedBy`](https://biomejs.dev/linter/rules/no-svelte-useless-derived-by/) fix preserves comma expressions as a single argument and keeps callback scope, explicit callback types, and comments intact. The rule keeps callbacks referenced in computed method names, recognizes when nested functions bind their own `this` or `arguments` in parameter defaults, and distinguishes potentially direct `eval` calls from merely returning `eval`.
