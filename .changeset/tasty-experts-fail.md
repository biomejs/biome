---
"@biomejs/biome": patch
---

Added two new options to [`noShadow`](https://biomejs.dev/linter/rules/no-shadow/), both defaulting to `true` to match typescript-eslint's behavior.

Fixed [#9482](https://github.com/biomejs/biome/issues/9482): Added `ignoreFunctionTypeParameterNameValueShadow` option. When enabled, parameter names inside function type annotations (e.g. `(options: unknown) => void`) are not flagged as shadowing outer variables.

Fixed [#7812](https://github.com/biomejs/biome/issues/7812): Added `ignoreTypeValueShadow` option. When enabled, a value binding that shares its name with a type-only declaration (type alias or interface) is not flagged, since types and values occupy separate namespaces in TypeScript.
