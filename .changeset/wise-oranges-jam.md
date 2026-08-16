---
"@biomejs/biome": patch
---

Fixed SVG parsing for files with an XML declaration followed by a `PUBLIC` doctype, such as `<?xml version="1.0"?><!DOCTYPE svg PUBLIC "a" "b">`.
