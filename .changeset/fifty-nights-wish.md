---
"@biomejs/biome": patch
---

Fixed a project scan that could take extremely long in monorepos with densely-linked module graphs, such as GraphQL schemas or barrel files. During dependency scanning Biome now opens each file at most once instead of re-opening it for every incoming import, which previously caused the same file to be indexed thousands of times.

