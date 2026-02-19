---
"@biomejs/biome": patch
---

Fixed [#9138](https://github.com/biomejs/biome/issues/9138): The HTML parser incorrectly failing to parse bracket characters (`[` and `]`) in text content (e.g. `<div>[Foo]</div>`).
