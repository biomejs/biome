---
"@biomejs/biome": patch
---

Fixed [#7727](https://github.com/biomejs/biome/issues/7727): GritQL `import $what from $where` patterns now match namespace imports and import clauses with multiple named specifiers, bringing Biome's matcher back in line with `grit apply`.
