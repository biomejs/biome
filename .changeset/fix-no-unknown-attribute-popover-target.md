---
"@biomejs/biome": patch
---

Fix a false positive in `noUnknownAttribute` where `popoverTarget` and `popoverTargetAction` were reported as invalid when no package manifest was available (for example, in the playground). The rule now accepts the exact camelCase spellings as-is without a manifest, while the existing gated behaviour for a manifest pinning a pre-19 React is unchanged. Other case variants still go through the existing suggestion logic.
