---
"@biomejs/biome": patch
---

Fixed [#9097](https://github.com/biomejs/biome/issues/9097): `organizeImports` no longer misapplies `:BLANK_LINE:` group separators inside Astro, Vue, and Svelte embedded script/frontmatter snippets.

Biome now trims delimiter-adjacent whitespace consistently when extracting embedded JavaScript and TypeScript, so import sorting behaves the same as it does in standalone JS and TS files.
