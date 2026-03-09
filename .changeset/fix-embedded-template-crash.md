---
"@biomejs/biome": patch
---

Fixed [#9131](https://github.com/biomejs/biome/issues/9131), [#9112](https://github.com/biomejs/biome/issues/9112), [#9166](https://github.com/biomejs/biome/issues/9166): the formatter no longer crashes or produces corrupt output when a JS file with `experimentalEmbeddedSnippetsEnabled` contains non-embedded template literals alongside embedded ones (e.g. `console.log(\`test\`)` next to `graphql(\`...\`)`).
