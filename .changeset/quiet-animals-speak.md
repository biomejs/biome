---
"@biomejs/biome": patch
---

Fixed a bug where a suppression comment with an empty explanation was valid.

Now a suppression comment `// biome-ignore lint:` will raise a **warning** diagnostic.
