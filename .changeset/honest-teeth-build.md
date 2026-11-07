---
"@biomejs/biome": minor
---

Added support for Astro and Starlight to [`noUndeclaredCustomProperties`](https://biomejs.dev/linter/rules/no-undeclared-custom-properties/). The rule now finds the custom properties of the stylesheets that Starlight adds to every page, including the ones listed in its `customCss` option, and the `cssVariable` of Astro fonts, so `var(--sl-color-text)` is no longer reported in a Starlight site. When Biome can't find all these stylesheets, for example because the Astro configuration computes them with a function, the rule doesn't report anything in that package.
