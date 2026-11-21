---
"@biomejs/biome": patch
---

Fixed `biome migrate eslint --write`, `biome migrate prettier --write`, and `biome migrate stylelint --write` writing `"extends": "//"` as `"extends": null`. Biome then reported the nested configuration file as a second root configuration.
