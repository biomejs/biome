---
"@biomejs/biome": patch
---

Add Markdown parser support for inline elements, block quotes, lists, headers, and code blocks.

Implements parsing for:
- ATX headers (`#` through `######`) with proper trailing hash support
- Fenced code blocks (``` and ~~~) with language tags
- Indented code blocks (4+ space indentation)
- Block quotes (`>`)
- Bullet lists (`-` and `*` markers)
- Inline elements: code spans, emphasis (bold), italic, links, and images
- Escape sequences (`\*`, `\[`, etc.) per CommonMark spec

Note: This is the initial parser implementation. Ordered lists, multi-line list items, and some advanced CommonMark features are not yet supported.
