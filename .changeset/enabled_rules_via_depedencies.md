---
cli: minor
---

# Enabled rules via dependencies

Biome now automatically enables some lint rules by scanning the closest `package.json`:

- The dependency `react` enables: `useExhaustiveDependencies` and `useHookAtTopLevel`.
- The dependency `next` enables: `noImgElement`, `noDocumentImportInPage`, `noHeadImportInDocument`.
- The dependency `solid` enables: `noReactSpecificProps`.
