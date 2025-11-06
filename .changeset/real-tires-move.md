---
"@biomejs/biome": patch
---

Refactored `FormatElement::Text` and `FormatElement::LocatedTokenText` to remove source_position and use the new
`FormatElement::SourcePosition` for source tracking, improving memory layout.
