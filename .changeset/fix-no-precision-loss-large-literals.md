---
"@biomejs/biome": patch
---

Fixed [#12996](https://github.com/biomejs/biome/issues/12996): `noPrecisionLoss` no longer reports decimal literals that don't lose precision, even when they have more than 17 significant digits. Exact values such as `18446744073709551616` (2^64) and `9223372036854775808` (2^63) are now accepted, while literals that actually lose precision at runtime (e.g. `18446744073709551617`) are still reported.
