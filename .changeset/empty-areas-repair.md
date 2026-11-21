---
"@biomejs/biome": patch
---

Fixed [#10501](https://github.com/biomejs/biome/issues/10501): Biome now selects the musl Linux binary in containers without `ldd`, while preserving glibc binary selection for statically linked glibc Node.js builds.
