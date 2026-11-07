---
"@biomejs/biome": minor
---

Added support for Astro and Starlight to [`noUndeclaredClasses`](https://biomejs.dev/linter/rules/no-undeclared-classes/). The rule now finds the classes of the stylesheets that Starlight adds to every page, including the ones listed in its `customCss` option, and of the global styles of Starlight's components, so `class="sl-flex"` is no longer reported in a Starlight site. When Biome can't find all these stylesheets, for example because the Astro configuration computes them with a function, the rule doesn't report anything in that package.
