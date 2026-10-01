---
"@biomejs/biome": minor
---

Added support for Google Apps Script (`.gs`) files.

They are parsed as JavaScript scripts, so ES module `import`/`export` statements are reported as errors (Apps Script has no module system). Common Apps Script service globals (e.g. `SpreadsheetApp`, `DriveApp`, `Logger`) are recognized by `noUndeclaredVariables` and `noGlobalAssign`, and the list can be extended through the `javascript.globals` option.
