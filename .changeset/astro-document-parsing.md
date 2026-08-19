---
"@biomejs/biome": patch
---

Fixed [#8294](https://github.com/biomejs/biome/issues/8294): comments inside an Astro expression no longer break the parse.

Fixed a number of other Astro constructs that were rejected or read incorrectly: `is:raw` children, interpolations inside a `<textarea>`, unquoted and template literal attribute values, attribute names starting with `:`, a bare `<` in text, `{{` at the start of an expression, `{}` inside MathML, `v-` prefixed attribute names, and `---` sequences that do not open frontmatter.
