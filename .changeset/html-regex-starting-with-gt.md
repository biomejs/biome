---
"@biomejs/biome": patch
---

Fixed the HTML parser treating a regex literal that starts with `>` as the end of a self-closing tag. Astro frontmatter such as `const escaped = s.replace(/>/g, "&gt;");` no longer swallows the closing `---` fence, and the same regex inside a template expression no longer runs past its closing `}`.
