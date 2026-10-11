---
"@biomejs/biome": minor
---

Promoted 59 nursery rules to stable groups.

#### Correctness

Promoted the following rules to the `correctness` group with error severity. All of them are recommended:

- [`noDuplicateFieldDefinitionNames`](https://biomejs.dev/linter/rules/no-duplicate-field-definition-names/)
- [`noVueRefAsOperand`](https://biomejs.dev/linter/rules/no-vue-ref-as-operand/)
- [`noVueImportCompilerMacros`](https://biomejs.dev/linter/rules/no-vue-import-compiler-macros/)
- [`noVueVOnNumberValues`](https://biomejs.dev/linter/rules/no-vue-v-on-number-values/)
- [`useVueValidVFor`](https://biomejs.dev/linter/rules/use-vue-valid-v-for/)
- [`noComponentHookFactories`](https://biomejs.dev/linter/rules/no-component-hook-factories/)
- [`noJsxNamespace`](https://biomejs.dev/linter/rules/no-jsx-namespace/)
- [`useReactAsyncServerFunction`](https://biomejs.dev/linter/rules/use-react-async-server-function/)
- [`noReactNativeRawText`](https://biomejs.dev/linter/rules/no-react-native-raw-text/)
- [`useReactNativePlatformComponents`](https://biomejs.dev/linter/rules/use-react-native-platform-components/)
- [`useQwikLoaderLocation`](https://biomejs.dev/linter/rules/use-qwik-loader-location/)
- [`noPlaywrightMissingAwait`](https://biomejs.dev/linter/rules/no-playwright-missing-await/)
- [`usePlaywrightValidDescribeCallback`](https://biomejs.dev/linter/rules/use-playwright-valid-describe-callback/)

#### Suspicious

Promoted the following rules to the `suspicious` group:

- [`noLoopFunc`](https://biomejs.dev/linter/rules/no-loop-func/)
- [`noJsxLeakedDollar`](https://biomejs.dev/linter/rules/no-jsx-leaked-dollar/)
- [`noReactStringRefs`](https://biomejs.dev/linter/rules/no-react-string-refs/) (recommended)
- [`useScopedStyles`](https://biomejs.dev/linter/rules/use-scoped-styles/) (recommended)
- [`noReactNativeDeepImports`](https://biomejs.dev/linter/rules/no-react-native-deep-imports/) (recommended, error severity)
- [`noDrizzleDeleteWithoutWhere`](https://biomejs.dev/linter/rules/no-drizzle-delete-without-where/) (recommended, error severity)
- [`noDrizzleUpdateWithoutWhere`](https://biomejs.dev/linter/rules/no-drizzle-update-without-where/) (recommended, error severity)
- [`noConditionalExpect`](https://biomejs.dev/linter/rules/no-conditional-expect/)
- [`useExpect`](https://biomejs.dev/linter/rules/use-expect/)
- [`noIdenticalTestTitle`](https://biomejs.dev/linter/rules/no-identical-test-title/) (recommended)
- [`noPlaywrightForceOption`](https://biomejs.dev/linter/rules/no-playwright-force-option/)
- [`noPlaywrightNetworkidle`](https://biomejs.dev/linter/rules/no-playwright-networkidle/)
- [`noPlaywrightPagePause`](https://biomejs.dev/linter/rules/no-playwright-page-pause/)
- [`noPlaywrightWaitForNavigation`](https://biomejs.dev/linter/rules/no-playwright-wait-for-navigation/)
- [`noPlaywrightWaitForTimeout`](https://biomejs.dev/linter/rules/no-playwright-wait-for-timeout/)
- [`noDuplicateSelectors`](https://biomejs.dev/linter/rules/no-duplicate-selectors/)
- [`noEmptyObjectKeys`](https://biomejs.dev/linter/rules/no-empty-object-keys/)
- [`useBaseline`](https://biomejs.dev/linter/rules/use-baseline/)
- [`noUntrustedLicenses`](https://biomejs.dev/linter/rules/no-untrusted-licenses/)

#### Security

Promoted [`useIframeSandbox`](https://biomejs.dev/linter/rules/use-iframe-sandbox/) to the `security` group with error severity.

#### Performance

Promoted the recommended rule [`useDomNodeTextContent`](https://biomejs.dev/linter/rules/use-dom-node-text-content/) to the `performance` group with warning severity.

#### Complexity

Promoted the following rules to the `complexity` group:

- [`useArraySome`](https://biomejs.dev/linter/rules/use-array-some/)
- [`useRegexpTest`](https://biomejs.dev/linter/rules/use-regexp-test/)
- [`noUnnecessaryTemplateExpression`](https://biomejs.dev/linter/rules/no-unnecessary-template-expression/)
- [`noPlaywrightUselessAwait`](https://biomejs.dev/linter/rules/no-playwright-useless-await/)
- [`noExcessiveSelectorClasses`](https://biomejs.dev/linter/rules/no-excessive-selector-classes/)
- [`useMathMinMax`](https://biomejs.dev/linter/rules/use-math-min-max/) (recommended, warning severity)
- [`noExcessiveNestedCallbacks`](https://biomejs.dev/linter/rules/no-excessive-nested-callbacks/)

#### Style

Promoted the following rules to the `style` group with information severity:

- [`useVueConsistentDefinePropsDeclaration`](https://biomejs.dev/linter/rules/use-vue-consistent-define-props-declaration/)
- [`useVueNextTickPromise`](https://biomejs.dev/linter/rules/use-vue-next-tick-promise/) (recommended)
- [`noReactNativeLiteralColors`](https://biomejs.dev/linter/rules/no-react-native-literal-colors/)
- [`useConsistentTestIt`](https://biomejs.dev/linter/rules/use-consistent-test-it/)
- [`useTestHooksInOrder`](https://biomejs.dev/linter/rules/use-test-hooks-in-order/)
- [`useTestHooksOnTop`](https://biomejs.dev/linter/rules/use-test-hooks-on-top/)
- [`noPlaywrightEval`](https://biomejs.dev/linter/rules/no-playwright-eval/)
- [`noPlaywrightElementHandle`](https://biomejs.dev/linter/rules/no-playwright-element-handle/)
- [`noPlaywrightWaitForSelector`](https://biomejs.dev/linter/rules/no-playwright-wait-for-selector/)
- [`useReduceTypeParameter`](https://biomejs.dev/linter/rules/use-reduce-type-parameter/)
- [`useNamedCaptureGroup`](https://biomejs.dev/linter/rules/use-named-capture-group/)
- [`useUnicodeRegex`](https://biomejs.dev/linter/rules/use-unicode-regex/)
- [`useImportsFirst`](https://biomejs.dev/linter/rules/use-imports-first/)
- [`useVarsOnTop`](https://biomejs.dev/linter/rules/use-vars-on-top/)
- [`useThisInClassMethods`](https://biomejs.dev/linter/rules/use-this-in-class-methods/)
- [`noInlineStyles`](https://biomejs.dev/linter/rules/no-inline-styles/)
- [`useDomQuerySelector`](https://biomejs.dev/linter/rules/use-dom-query-selector/)
- [`noTopLevelLiterals`](https://biomejs.dev/linter/rules/no-top-level-literals/)
