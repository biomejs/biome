---
"@biomejs/biome": patch
---

Fixed [#13429](https://github.com/biomejs/biome/issues/13429): `^0.0.0` and `^0.0` in `package.json` no longer match every `0.x` release. `^0.0.0` stays below `0.0.1`, and `^0.0` stays below `0.1.0`. A stub such as `"react-native": "^0.0.0"` no longer enables the React Native lint domain.
