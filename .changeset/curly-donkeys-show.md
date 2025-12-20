---
"@biomejs/biome": patch
---

Add Markdown parser support for inline elements, block quotes, lists, headers, and code blocks.

Implements parsing for:
- ATX headers (`#` through `######`)
- Fenced code blocks with language tags
- Indented code blocks (4+ space indentation)
- Block quotes (`>`)
- Bullet lists (`-` and `*` markers)
- Inline elements: code spans, emphasis (bold), italic, links, and images

Note: This is the initial parser implementation. Ordered lists, multi-line list items, escape sequences, and some advanced CommonMark features are not yet supported.
