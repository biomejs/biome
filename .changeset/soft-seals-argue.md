---
"@biomejs/biome": minor
---

Added support for Starlight to [`noUnusedClasses`](https://biomejs.dev/linter/rules/no-unused-classes/). A class of a stylesheet listed in the `customCss` option of Starlight is now used when any file of the package or any Starlight component references it, so a `.sidebar-content` selector that only Starlight's layout uses is no longer reported. The rule doesn't report anything in the package when Biome can't find all the stylesheets that Starlight adds, for example because a function computes `customCss`, and doesn't report the classes of these stylesheets when the package contains Markdown or MDX pages, or Astro, Vue, and Svelte files that Biome reads without full HTML support.
