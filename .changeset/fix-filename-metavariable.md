---
"@biomejs/biome": patch
---

fix(grit): make $filename metavariable accessible in GritQL plugins

Fixed [#6670](https://github.com/biomejs/biome/issues/6670). The `$filename` metavariable can now be used in GritQL `where` clauses to filter matches by filename.
