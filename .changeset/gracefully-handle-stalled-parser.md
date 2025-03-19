---
"@biomejs/biome": patch
---

Fixed [#4622](https://github.com/biomejs/biome/issues/4622): Our JavaScript parser can now gracefully handle situations where we detect the parser to have stalled.

This means we don't fail with an assertion anymore, but invalid code can trigger a regular diagnostic in such cases.
