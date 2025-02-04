---
"@biomejs/biome": major
---

Remove `indentSize` deprecated option.

The deprecated option `indentSize`, and its relative CLI options, has been removed:
- Configuration file: `formatter.indentSize`
- Configuration file: `javascript.formatter.indentSize`
- Configuration file: `json.formatter.indentSize`
- CLI option `--indent-size`
- CLI option `--javascript-formatter-indent-size`
- CLI option `--json-formatter-indent-size`

Use `indentWidth` and its relative CLI options instead.
