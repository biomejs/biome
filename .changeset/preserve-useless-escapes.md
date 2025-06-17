---
"@biomejs/biome": major
---

[Prettier 3.4](https://prettier.io/blog/2024/11/26/3.4.0.html) introduced a change in their normalization process of string literals: it no longer unescapes useless escape sequences.
Biome now matches the new behavior of Prettier when formatting code.
This affects the JSON and JavaScript formatters.
