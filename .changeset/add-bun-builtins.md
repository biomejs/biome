---
"@biomejs/biome": patch
---

Fixed [#11475](https://github.com/biomejs/biome/issues/11475): the resolver now recognizes Bun runtime built-in modules (`bun`, `bun:test`, `bun:sqlite`, `bun:ffi`, `bun:jsc`, `bun:wrap`), so `noUnresolvedImports` no longer reports them as unresolved. They are kept in a dedicated list, separate from Node.js built-ins, so `noNodejsModules` and `useNodejsImportProtocol` correctly continue to treat them as **not** Node.js modules.
