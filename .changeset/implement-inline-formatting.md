---
"@biomejs/biome": patch
---

Implemented formatting for Markdown inline code, emphasis, and italic elements. These nodes previously fell back to verbatim output and now format their delimiters and content through the standard pipeline.
