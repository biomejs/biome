---
"@biomejs/biome": patch
---

Enabled HTML parsing for `.svg` files. SVG files are valid HTML and can now be processed by Biome's HTML toolchain when `html.experimental_full_support_enabled` is set to `true`.
