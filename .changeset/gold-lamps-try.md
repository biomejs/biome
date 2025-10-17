---
"@biomejs/backend-jsonrpc": patch
"@biomejs/biome": patch
"@biomejs/js-api": patch
---

Increase noExcessiveLinesPerFunction max line option to uint16 (up to 65 535) so JSX/front-end projects aren’t blocked by the old 255 cap
