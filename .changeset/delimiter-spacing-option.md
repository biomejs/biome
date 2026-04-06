---
"@biomejs/biome": minor
---

Added the `delimiterSpacing` formatter option. This option inserts spaces inside delimiters (after the opening delimiter and before the closing delimiter) when the content fits on a single line. Empty delimiters are not affected, and no space is added before the opening delimiter. The specific delimiters affected depend on the language. It can be configured globally via `formatter.delimiterSpacing` or per-language via `javascript.formatter.delimiterSpacing`, `json.formatter.delimiterSpacing`, and `css.formatter.delimiterSpacing`. Defaults to false.
