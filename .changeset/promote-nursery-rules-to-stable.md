---
"@biomejs/biome": minor
---

Promoted 21 nursery rules to stable groups.

#### Correctness

Promoted the following rules to the `correctness` group:

- [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/). The rule reports imports that cannot be resolved.
  The default rule severity is set to `error`.
- [`noVueReservedProps`](https://biomejs.dev/linter/rules/no-vue-reserved-props/). The rule reports Vue reserved props usage.
  The default rule severity is set to `error`.
- [`noVueReservedKeys`](https://biomejs.dev/linter/rules/no-vue-reserved-keys/). The rule reports Vue reserved keys usage.
  The default rule severity is set to `error`.
- [`noVueDataObjectDeclaration`](https://biomejs.dev/linter/rules/no-vue-data-object-declaration/). The rule reports Vue 2 data declared as an object instead of a function.
  The default rule severity is set to `warn`.
- [`noNextAsyncClientComponent`](https://biomejs.dev/linter/rules/no-next-async-client-component/). The rule reports async Next.js client components.
  The default rule severity is set to `warn`.
- [`noVueDuplicateKeys`](https://biomejs.dev/linter/rules/no-vue-duplicate-keys/). The rule reports duplicate keys in Vue component options.
  The default rule severity is set to `error`.
- [`noVueSetupPropsReactivityLoss`](https://biomejs.dev/linter/rules/no-vue-setup-props-reactivity-loss/). The rule reports destructuring of props in Vue 3 setup which causes reactivity loss.
  The default rule severity is set to `error`.
- [`useQwikMethodUsage`](https://biomejs.dev/linter/rules/use-qwik-method-usage/). The rule enforces correct Qwik framework method usage.
  The default rule severity is set to `error`.
- [`useQwikValidLexicalScope`](https://biomejs.dev/linter/rules/use-qwik-valid-lexical-scope/). The rule enforces valid lexical scope in Qwik framework.
  The default rule severity is set to `error`.

#### Suspicious

Promoted the following rules to the `suspicious` group:

- [`noImportCycles`](https://biomejs.dev/linter/rules/no-import-cycles/). The rule reports circular imports.
  The default rule severity is set to `warn`.
- [`noDeprecatedImports`](https://biomejs.dev/linter/rules/no-deprecated-imports/). The rule reports imports of deprecated symbols.
  The default rule severity is set to `warn`.
- [`noReactForwardRef`](https://biomejs.dev/linter/rules/no-react-forward-ref/). The rule reports usage of `React.forwardRef`.
  The default rule severity is set to `warn`.
- [`noUnusedExpressions`](https://biomejs.dev/linter/rules/no-unused-expressions/). The rule reports expressions that are never used.
  The default rule severity is set to `warn`.
- [`noEmptySource`](https://biomejs.dev/linter/rules/no-empty-source/). The rule reports empty source files.
  The default rule severity is set to `warn`.
- [`useDeprecatedDate`](https://biomejs.dev/linter/rules/use-deprecated-date/). The rule enforces use of GraphQL `@deprecated` directive with date.
  The default rule severity is set to `warn`.
- [`noDuplicateDependencies`](https://biomejs.dev/linter/rules/no-duplicate-dependencies/). The rule reports duplicate dependencies in package.json.
  The default rule severity is set to `warn`.

#### Complexity

Promoted the following rules to the `complexity` group:

- [`noUselessUndefined`](https://biomejs.dev/linter/rules/no-useless-undefined/). The rule reports useless `undefined` initialization and returns.
  The default rule severity is set to `info`.
- [`useMaxParams`](https://biomejs.dev/linter/rules/use-max-params/). The rule enforces a maximum number of function parameters.
  The default rule severity is set to `warn`.
- [`noUselessCatchBinding`](https://biomejs.dev/linter/rules/no-useless-catch-binding/). The rule reports useless catch binding parameters.
  The default rule severity is set to `info`.

#### Style

Promoted the following rules to the `style` group:

- [`useConsistentArrowReturn`](https://biomejs.dev/linter/rules/use-consistent-arrow-return/). The rule enforces consistent return in arrow functions.
  The default rule severity is set to `info`.
- [`noJsxLiterals`](https://biomejs.dev/linter/rules/no-jsx-literals/). The rule reports literal strings in JSX.
  The default rule severity is set to `info`.
