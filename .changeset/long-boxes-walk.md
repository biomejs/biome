---
"@biomejs/biome": patch
---

Fixed a resolver bug where packages that define a typed entry point through `package.json`'s `main` field but omit `types` were ignored during type-aware resolution. Type-aware rules such as [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) can now inspect imports from those packages.
