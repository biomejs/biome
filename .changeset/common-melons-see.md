---
"@biomejs/biome": patch
---

Fixed [#6360](https://github.com/biomejs/biome/issues/6360): The following pseudo classes and elements are no longer reported by `noUnknownPseudoClass` or `noUnknownPseudoElement` rules.

- `:open`
- `::details-content`
- `::prefix`
- `::search-text`
- `::suffix`
