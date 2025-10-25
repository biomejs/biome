---
"@biomejs/biome": patch
---

Fixed incorrect option name in HTML parser error message.

The error message for disabled text expressions incorrectly referred
to the `html.parser.textExpression` option, which does not exist.
Updated it to reference the correct `html.parser.interpolation` option.
