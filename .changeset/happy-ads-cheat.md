---
"@biomejs/biome": patch
---

Fixed [#8986](https://github.com/biomejs/biome/issues/8986): Biome's language server now scopes all watched-file patterns to each workspace folder, falling back to the deprecated `rootUri` when no workspace folders are provided. Clients without relative-pattern support receive compatible absolute glob patterns.
