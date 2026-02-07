---
"@biomejs/biome": patch
---

Enhanced `noSkippedTests` to detect Playwright patterns (`.fixme`, `test.describe`, `test.step`, bracket notation, bare calls). Added `allowConditional` option. Consolidated `noPlaywrightSkippedTest` into this rule.
