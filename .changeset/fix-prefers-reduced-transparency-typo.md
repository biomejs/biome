---
"@biomejs/biome": patch
---

Fix [`noUnknownMediaFeatureName`](https://biomejs.dev/linter/rules/no-unknown-media-feature-name/) false positive for `prefers-reduced-transparency` media feature. The feature name was misspelled as `prefers-reduded-transparency` in the keywords list.
