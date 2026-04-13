---
"@biomejs/biome": patch
---

Fix `--reporter=json` and `--reporter=rdjson` producing invalid JSON on Windows due to unescaped backslashes in file paths. Path separators are now normalized to forward slashes in JSON output, making the output consistent across platforms.
