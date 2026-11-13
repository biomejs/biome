---
"@biomejs/biome": patch
---

Fixed parsing of Markdown block quote lines that have only spaces or tabs after the `>`. Biome no longer hangs when a list item contains a block quote with a list and a later line of that quote has two or more spaces after the `>`, and such a line no longer makes the list inside the quote loose. Directly inside a block quote, such a line is now blank, like a bare `>` line, instead of a hard line break or the start of an indented code block.
