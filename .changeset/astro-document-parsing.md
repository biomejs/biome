---
"@biomejs/biome": patch
---

Fixed a number of Astro constructs that were rejected or read incorrectly: comments inside an expression, expressions inside `<pre>` and `<textarea>`, `is:raw` children, unquoted and template literal attribute values, attribute names starting with `:`, a bare `<` in text, `{{` at the start of an expression, `{}` inside MathML, `v-` prefixed attribute names, and `---` sequences that do not open frontmatter.

Variables used only inside a `<pre>` or `<textarea>` expression are no longer reported as unused.
