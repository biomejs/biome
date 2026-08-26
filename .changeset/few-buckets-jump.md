---
"@biomejs/biome": patch
---

Fixed [#11390](https://github.com/biomejs/biome/issues/11390), where `noFloatingPromises` performed expensive full type inference for calls to non-Promise methods declared on third-party TypeScript classes. The rule now classifies those calls using targeted type information.
