---
"@biomejs/biome": patch
---

Fixed the `FileFeaturesResult` interface in the WASM API was defined as a mapped object but the actual value was a `Map` object.
