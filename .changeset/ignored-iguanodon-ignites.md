---
"@biomejs/biome": minor
---

Fixed minor inconsistencies in how `files.includes` was being handled.

Previously, Biome sometimes failed to properly ignore the contents of a folder if you didn't specify the `/**` at the end of a glob pattern. This was unfortunate, because it meant we still had to traverse the folder and then apply the glob to every entry inside it.

This is no longer an issue and we now recommend to ignore folders without using the `/**` suffix.
