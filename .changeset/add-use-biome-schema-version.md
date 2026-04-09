---
"@biomejs/biome": minor
---

Added a `useBiomeSchemaVersion` lint rule that detects mismatched `$schema` URLs in `biome.json` and `biome.jsonc` and provides a safe quick fix to update the version segment to match the running CLI version.

Consolidated schema-version reporting by removing the duplicate deserialization diagnostic that previously emitted the same message during configuration deserialization. The lint is now the single source of truth for this diagnostic and provides the recommended quick-fix.
