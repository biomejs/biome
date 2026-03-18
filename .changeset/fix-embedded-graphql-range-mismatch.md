---
"@biomejs/biome": patch
---

Fixed [#9484](https://github.com/biomejs/biome/issues/9484): the formatter no longer crashes when formatting JS/TS files with embedded GraphQL tagged template literals (`graphql`...``) after parenthesized expressions. The syntax rewriter that removes unnecessary parentheses was shifting text positions, causing a range mismatch between the transformed and original trees. Embedded ranges are now correctly mapped back through the source map.
