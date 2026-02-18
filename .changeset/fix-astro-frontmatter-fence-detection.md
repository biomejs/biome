---
"@biomejs/biome": patch
---

Fixed [#8882](https://github.com/biomejs/biome/issues/8882) and [#9108](https://github.com/biomejs/biome/issues/9108): The Astro frontmatter lexer now correctly identifies the closing `---` fence when the frontmatter contains multi-line block comments with quote characters, strings that mix quote types (e.g. `"it's"`), or escaped quote characters (e.g. `"\"`).
