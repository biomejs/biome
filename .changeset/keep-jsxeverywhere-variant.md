---
"@biomejs/biome": patch
---

Fixed [#7286](https://github.com/biomejs/biome/issues/7286). Files are now formatted with JSX behavior when `javascript.parser.jsxEverywhere` is explicitly set.

Previously, this flag was only used for parsing, but not for formatting, which resulted in incorrect formatting of conditional expressions when JSX syntax is used in `.js` files.
