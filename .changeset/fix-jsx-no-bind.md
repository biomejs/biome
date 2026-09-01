---
"@biomejs/biome": patch
---

Fixed [#4637](https://github.com/biomejs/biome/issues/4637). The [`noJsxPropsBind`](https://biomejs.dev/linter/rules/no-jsx-props-bind/) rule now supports `allowArrowFunctions`, `allowFunctions`, `allowBind`, `ignoreDOMComponents` and `ignoreRefs` options, matching `eslint-plugin-react`'s `jsx-no-bind` behavior.
