---
"@biomejs/biome": patch
---

Fixed a parser diagnostic's message when vue syntax is disabled so that it no longer references the non-existant `html.parser.vue` option. This option will become available in 2.5.
