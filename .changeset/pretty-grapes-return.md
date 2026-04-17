---
"@biomejs/biome": minor
---

Added the `reactNative` domain to existing React lint rules that are relevant to React Native development. When a project has `react-native` as a dependency, these rules are now automatically enabled. The affected rules are:

- [`useHookAtTopLevel`](https://biomejs.dev/linter/rules/use-hook-at-top-level/)
- [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/)
- [`noChildrenProp`](https://biomejs.dev/linter/rules/no-children-prop/)
- [`noRenderReturnValue`](https://biomejs.dev/linter/rules/no-render-return-value/)
- [`noNestedComponentDefinitions`](https://biomejs.dev/linter/rules/no-nested-component-definitions/)
- [`noReactPropAssignments`](https://biomejs.dev/linter/rules/no-react-prop-assignments/)
- [`useJsxKeyInIterable`](https://biomejs.dev/linter/rules/use-jsx-key-in-iterable/)
- [`useUniqueElementIds`](https://biomejs.dev/linter/rules/use-unique-element-ids/)
- [`noCommentText`](https://biomejs.dev/linter/rules/no-comment-text/)
- [`noDuplicateJsxProps`](https://biomejs.dev/linter/rules/no-duplicate-jsx-props/)
- [`noArrayIndexKey`](https://biomejs.dev/linter/rules/no-array-index-key/)
- [`noReactForwardRef`](https://biomejs.dev/linter/rules/no-react-forward-ref/)
- [`noLeakedRender`](https://biomejs.dev/linter/rules/no-leaked-render/)
- [`noComponentHookFactories`](https://biomejs.dev/linter/rules/no-component-hook-factories/)
- [`noJsxPropsBind`](https://biomejs.dev/linter/rules/no-jsx-props-bind/)
- [`noDuplicatedSpreadProps`](https://biomejs.dev/linter/rules/no-duplicated-spread-props/)
- [`useComponentExportOnlyModules`](https://biomejs.dev/linter/rules/use-component-export-only-modules/)
- [`useReactFunctionComponents`](https://biomejs.dev/linter/rules/use-react-function-components/)
