---
"@biomejs/biome": patch
---

Fixed the Markdown parser treating a blockquote line that contains only spaces or tabs as an indented code block. The formatter now removes the extra whitespace after the `>` of such a line, as it does for other blank lines in blockquotes.
