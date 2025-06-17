---
"@biomejs/biome": major
---

The rule [`useImportExtensions`](https://biomejs.dev/linter/rules/use-import-extensions/) has been updated to suggest actual file extensions instead of guesses based on hueristics.

As part of this, the `suggestedExtensions` option has been removed. A simpler,
new option called `forceJsExtensions` has been introduced for those who use
`tsc`'s `"module": "node16"` setting.

The rule also no longer reports diagnostics to add an extension when the path
doesn't exist at all, with or without extension.
