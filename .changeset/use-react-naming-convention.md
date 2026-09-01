---
"@biomejs/biome": patch
---

Added the new nursery rule [`useReactNamingConvention`](https://biomejs.dev/linter/rules/use-react-naming-convention/), which enforces naming conventions for React values assigned from `createContext`, `useId`, and `useRef`. A value from `createContext` must be a PascalCase component name ending with `Context`, a value from `useId` must be named `id` or end with `Id`, and a value from `useRef` must be named `ref` or end with `Ref`.
