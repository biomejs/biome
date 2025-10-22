---
"@biomejs/biome": minor
---

Promoted new lint rules:
- Promoted `noNonNullAssertedOptionalChain` to the suspicious group
- Promoted `useReactFunctionComponents` to the `style` group
- Promoted `useImageSize` to the `correctness` group
- Promoted `useConsistentTypeDefinitions` to the `style` group
- Promoted `useQwikClasslist` to the `correctness` group
- Promoted `noSecrets` to the `security` group

Removed the lint rule `useAnchorHref`, because its use case is covered by `useValidAnchor`.
