---
"@biomejs/biome": patch
---

Fixed [#9138](https://github.com/biomejs/biome/issues/9138): Astro files containing `---` in HTML content (e.g., `<h1>---Hi</h1>`) are now parsed correctly, both when a frontmatter block is present and when there is no frontmatter at all.
