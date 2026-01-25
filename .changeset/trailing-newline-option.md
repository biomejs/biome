---
"@biomejs/biome": minor
---

Added the formatter option [`trailingNewline`](https://biomejs.dev/reference/configuration/#formattertrailingnewline).

When set to `false`, the formatter will remove the trailing newline at the end of formatted files. The default value is `true`, which preserves the current behavior of adding a trailing newline.

This option is available globally and for each language-specific formatter configuration:

```json
{
  "formatter": {
    "trailingNewline": false
  },
  "javascript": {
    "formatter": {
      "trailingNewline": true
    }
  }
}
```

The following CLI flags have been added. They accept `true` or `false` as value:
- `--formatter-trailing-newline`
- `--javascript-formatter-trailing-newline`
- `--json-formatter-trailing-newline`
- `--graphql-formatter-trailing-newline`
- `--css-formatter-trailing-newline`
- `--html-formatter-trailing-newline`

