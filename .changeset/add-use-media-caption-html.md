---
"@biomejs/biome": patch
---

Added the HTML-specific lint rule [`useMediaCaption`](https://biomejs.dev/linter/rules/use-media-caption/). Enforces that `audio` and `video` elements have a `track` element with `kind="captions"` for accessibility. Muted videos are allowed without captions.
