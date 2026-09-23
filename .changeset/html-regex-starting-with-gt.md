---
"@biomejs/biome": patch
---

Fixed [withastro/compiler-rs#194](https://github.com/withastro/compiler-rs/issues/194): the HTML parser no longer treats a regex literal that starts with `>` as the end of a self-closing tag. Astro frontmatter such as `const escaped = s.replace(/>/g, "&gt;");` no longer swallows the closing `---` fence, and the same regex inside a template expression no longer runs past its closing `}`. A regex literal after a keyword such as `return`, as in `return />'/.test(s)`, is now recognized too.
