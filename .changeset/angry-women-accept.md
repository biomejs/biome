---
"@biomejs/biome": minor
---

The Biome CSS parser is now able to parse Vue SFC syntax such as `:slotted` and `:deep`. These pseudo functions are only correctly parsed when the CSS is defined inside `.vue` components. Otherwise, Biome will a emit a parse error.

This capability is only available when `experimentalFullHtmlSupportedEnabled` is set to `true`.
