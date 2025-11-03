---
"@biomejs/biome": patch
---

Fixed `noExtraNonNullAssertion` incorrectly flagging separate non-null assertions on both sides of an assignment (e.g. `arr[0]! ^= arr[1]!`).

The rule now correctly distinguishes between nested non-null assertions (still flagged, e.g. `arr[0]!! ^= arr[1]` or `arr[0] ^= arr[1]!!`) and separate non-null assertions on different sides of an assignment (allowed).
