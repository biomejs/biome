---
"@biomejs/biome": major
---

Biome no longer treats too large files as errors.

Previously, files that exceed the configured size limit would throw an error, and the CLI would exit with an error code.

Now, the CLI ignores the file, emits an *information* diagnostic and doesn't exit with an error code.
