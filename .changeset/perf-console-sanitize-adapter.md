---
"@biomejs/biome": patch
---

Improved the performance of printing diagnostics to the console. `SanitizeAdapter` (used by every CLI diagnostic and log line) previously wrote its output one character at a time, each going through a separate write call; it now batches runs of unmodified content into a single write, only breaking the run to substitute a character that actually needs it (e.g. a zero-width character, or a non-ASCII character on Windows). Linting a large project with many diagnostics could previously spend most of its wall-clock time in this write loop rather than in analysis.
