---
"@biomejs/biome": patch
---

Fixed [#10742](https://github.com/biomejs/biome/issues/10742): Solid domain detection now uses the correct package name `solid-js`. Previously, projects listing `solid-js` as a dependency did not enable Solid-domain rules; the registry entry was the bare `solid` literal.
