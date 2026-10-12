---
"@biomejs/biome": patch
---

Fixed the parsing of Svelte `class:` and `style:` directive names. Modifiers such as `|important` in `style:color|important` are now parsed as modifiers instead of being part of the name, and `/` in `<input class:invalid/>` now closes the tag.
