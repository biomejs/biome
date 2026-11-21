---
"@biomejs/biome": patch
---

Fixed [`noUselessEscapeInString`](https://biomejs.dev/linter/rules/no-useless-escape-in-string/) reporting `\#{` in SCSS strings. The backslash stops Sass from interpolating, so the rule's safe fix changed the compiled CSS: `content: "\#{foo}"` compiles to `"#{foo}"`, but without the backslash Sass evaluates `foo`.
