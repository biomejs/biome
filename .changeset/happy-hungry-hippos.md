---
"@biomejs/biome": minor
---

Introduced a new configuration setting `files.experimentalScannerIgnores`.

This setting may be used to configure a set of file and folder names that should
be unconditionally ignored by Biome's scanner.

Biome maintains an internal list of default ignore entries, which is based on
user feedback and which may change in any release. This setting allows
overriding this internal list completely.

This is considered an advanced feature that users _should_ not need to tweak
themselves, but they can as a last resort. This setting can only be configured
in root configurations, and is ignored in nested configs.
