---
"@biomejs/biome": patch
---

Fixed [#12724](https://github.com/biomejs/biome/issues/12724): strong emphasis in paragraphs now reflows soft line breaks when `markdown.formatter.proseWrap` is set to `always`. Explicit hard breaks are preserved, while quoted text and fence-like sequences keep their Markdown meaning.
