---
"@biomejs/biome": patch
---

Fix incorrect option name in HTML parser error message.

The error message for disabled text expressions incorrectly referred
to the `html.parser.textExpression` option, which does not exist.
Update it to reference the correct `html.parser.interpolation` option.
