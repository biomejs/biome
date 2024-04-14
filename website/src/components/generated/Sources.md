## Clippy rules to Biome
| Clippy rule name | Biome rule name |
| ---- | ---- |
| [approx_constant](https://rust-lang.github.io/rust-clippy/master/#/approx_constant) |[noApproximativeNumericConstant](/linter/rules//lint/rules/no-approximative-numeric-constant) |
| [match_str_case_mismatch](https://rust-lang.github.io/rust-clippy/master/#/match_str_case_mismatch) |[noStringCaseMismatch](/linter/rules//lint/rules/no-string-case-mismatch) |
| [misrefactored_assign_op](https://rust-lang.github.io/rust-clippy/master/#/misrefactored_assign_op) |[noMisrefactoredShorthandAssign](/linter/rules//lint/rules/no-misrefactored-shorthand-assign) |
## ESLint rules to Biome
| ESLint rule name | Biome rule name |
| ---- | ---- |
| [constructor-super](https://eslint.org/docs/latest/rules/constructor-super) |[noInvalidConstructorSuper](/linter/rules//lint/rules/no-invalid-constructor-super) |
| [curly](https://eslint.org/docs/latest/rules/curly) |[useBlockStatements](/linter/rules//lint/rules/use-block-statements) |
| [default-case-last](https://eslint.org/docs/latest/rules/default-case-last) |[useDefaultSwitchClauseLast](/linter/rules//lint/rules/use-default-switch-clause-last) |
| [default-param-last](https://eslint.org/docs/latest/rules/default-param-last) |[useDefaultParameterLast](/linter/rules//lint/rules/use-default-parameter-last) |
| [dot-notation](https://eslint.org/docs/latest/rules/dot-notation) |[useLiteralKeys](/linter/rules//lint/rules/use-literal-keys) |
| [eqeqeq](https://eslint.org/docs/latest/rules/eqeqeq) |[noDoubleEquals](/linter/rules//lint/rules/no-double-equals) |
| [for-direction](https://eslint.org/docs/latest/rules/for-direction) |[useValidForDirection](/linter/rules//lint/rules/use-valid-for-direction) |
| [getter-return](https://eslint.org/docs/latest/rules/getter-return) |[useGetterReturn](/linter/rules//lint/rules/use-getter-return) |
| [no-async-promise-executor](https://eslint.org/docs/latest/rules/no-async-promise-executor) |[noAsyncPromiseExecutor](/linter/rules//lint/rules/no-async-promise-executor) |
| [no-case-declarations](https://eslint.org/docs/latest/rules/no-case-declarations) |[noSwitchDeclarations](/linter/rules//lint/rules/no-switch-declarations) |
| [no-class-assign](https://eslint.org/docs/latest/rules/no-class-assign) |[noClassAssign](/linter/rules//lint/rules/no-class-assign) |
| [no-compare-neg-zero](https://eslint.org/docs/latest/rules/no-compare-neg-zero) |[noCompareNegZero](/linter/rules//lint/rules/no-compare-neg-zero) |
| [no-cond-assign](https://eslint.org/docs/latest/rules/no-cond-assign) |[noAssignInExpressions](/linter/rules//lint/rules/no-assign-in-expressions) |
| [no-console](https://eslint.org/docs/latest/rules/no-console) |[noConsoleLog](/linter/rules//lint/rules/no-console-log) |
| [no-const-assign](https://eslint.org/docs/latest/rules/no-const-assign) |[noConstAssign](/linter/rules//lint/rules/no-const-assign) |
| [no-constant-condition](https://eslint.org/docs/latest/rules/no-constant-condition) |[noConstantCondition](/linter/rules//lint/rules/no-constant-condition) |
| [no-constructor-return](https://eslint.org/docs/latest/rules/no-constructor-return) |[noConstructorReturn](/linter/rules//lint/rules/no-constructor-return) |
| [no-control-regex](https://eslint.org/docs/latest/rules/no-control-regex) |[noControlCharactersInRegex](/linter/rules//lint/rules/no-control-characters-in-regex) |
| [no-debugger](https://eslint.org/docs/latest/rules/no-debugger) |[noDebugger](/linter/rules//lint/rules/no-debugger) |
| [no-dupe-args](https://eslint.org/docs/latest/rules/no-dupe-args) |[noDuplicateParameters](/linter/rules//lint/rules/no-duplicate-parameters) |
| [no-dupe-class-members](https://eslint.org/docs/latest/rules/no-dupe-class-members) |[noDuplicateClassMembers](/linter/rules//lint/rules/no-duplicate-class-members) |
| [no-dupe-keys](https://eslint.org/docs/latest/rules/no-dupe-keys) |[noDuplicateObjectKeys](/linter/rules//lint/rules/no-duplicate-object-keys) |
| [no-duplicate-case](https://eslint.org/docs/latest/rules/no-duplicate-case) |[noDuplicateCase](/linter/rules//lint/rules/no-duplicate-case) |
| [no-else-return](https://eslint.org/docs/latest/rules/no-else-return) |[noUselessElse](/linter/rules//lint/rules/no-useless-else) |
| [no-empty-character-class](https://eslint.org/docs/latest/rules/no-empty-character-class) |[noEmptyCharacterClassInRegex](/linter/rules//lint/rules/no-empty-character-class-in-regex) |
| [no-empty-pattern](https://eslint.org/docs/latest/rules/no-empty-pattern) |[noEmptyPattern](/linter/rules//lint/rules/no-empty-pattern) |
| [no-ex-assign](https://eslint.org/docs/latest/rules/no-ex-assign) |[noCatchAssign](/linter/rules//lint/rules/no-catch-assign) |
| [no-extra-boolean-cast](https://eslint.org/docs/latest/rules/no-extra-boolean-cast) |[noExtraBooleanCast](/linter/rules//lint/rules/no-extra-boolean-cast) |
| [no-extra-label](https://eslint.org/docs/latest/rules/no-extra-label) |[noUselessLabel](/linter/rules//lint/rules/no-useless-label) |
| [no-fallthrough](https://eslint.org/docs/latest/rules/no-fallthrough) |[noFallthroughSwitchClause](/linter/rules//lint/rules/no-fallthrough-switch-clause) |
| [no-func-assign](https://eslint.org/docs/latest/rules/no-func-assign) |[noFunctionAssign](/linter/rules//lint/rules/no-function-assign) |
| [no-import-assign](https://eslint.org/docs/latest/rules/no-import-assign) |[noImportAssign](/linter/rules//lint/rules/no-import-assign) |
| [no-inner-declarations](https://eslint.org/docs/latest/rules/no-inner-declarations) |[noInnerDeclarations](/linter/rules//lint/rules/no-inner-declarations) |
| [no-label-var](https://eslint.org/docs/latest/rules/no-label-var) |[noLabelVar](/linter/rules//lint/rules/no-label-var) |
| [no-labels](https://eslint.org/docs/latest/rules/no-labels) |[noConfusingLabels](/linter/rules//lint/rules/no-confusing-labels) |
| [no-lonely-if](https://eslint.org/docs/latest/rules/no-lonely-if) |[useCollapsedElseIf](/linter/rules//lint/rules/use-collapsed-else-if) |
| [no-loss-of-precision](https://eslint.org/docs/latest/rules/no-loss-of-precision) |[noPrecisionLoss](/linter/rules//lint/rules/no-precision-loss) |
| [no-negated-condition](https://eslint.org/docs/latest/rules/no-negated-condition) |[noNegationElse](/linter/rules//lint/rules/no-negation-else) |
| [no-new-native-nonconstructor](https://eslint.org/docs/latest/rules/no-new-native-nonconstructor) |[noInvalidNewBuiltin](/linter/rules//lint/rules/no-invalid-new-builtin) |
| [no-new-symbol](https://eslint.org/docs/latest/rules/no-new-symbol) |[noNewSymbol](/linter/rules//lint/rules/no-new-symbol) |
| [no-nonoctal-decimal-escape](https://eslint.org/docs/latest/rules/no-nonoctal-decimal-escape) |[noNonoctalDecimalEscape](/linter/rules//lint/rules/no-nonoctal-decimal-escape) |
| [no-obj-calls](https://eslint.org/docs/latest/rules/no-obj-calls) |[noGlobalObjectCalls](/linter/rules//lint/rules/no-global-object-calls) |
| [no-param-reassign](https://eslint.org/docs/latest/rules/no-param-reassign) |[noParameterAssign](/linter/rules//lint/rules/no-parameter-assign) |
| [no-prototype-builtins](https://eslint.org/docs/latest/rules/no-prototype-builtins) |[noPrototypeBuiltins](/linter/rules//lint/rules/no-prototype-builtins) |
| [no-regex-spaces](https://eslint.org/docs/latest/rules/no-regex-spaces) |[noMultipleSpacesInRegularExpressionLiterals](/linter/rules//lint/rules/no-multiple-spaces-in-regular-expression-literals) |
| [no-restricted-globals](https://eslint.org/docs/latest/rules/no-restricted-globals) |[noRestrictedGlobals](/linter/rules//lint/rules/no-restricted-globals) |
| [no-self-assign](https://eslint.org/docs/latest/rules/no-self-assign) |[noSelfAssign](/linter/rules//lint/rules/no-self-assign) |
| [no-self-compare](https://eslint.org/docs/latest/rules/no-self-compare) |[noSelfCompare](/linter/rules//lint/rules/no-self-compare) |
| [no-sequences](https://eslint.org/docs/latest/rules/no-sequences) |[noCommaOperator](/linter/rules//lint/rules/no-comma-operator) |
| [no-setter-return](https://eslint.org/docs/latest/rules/no-setter-return) |[noSetterReturn](/linter/rules//lint/rules/no-setter-return) |
| [no-shadow-restricted-names](https://eslint.org/docs/latest/rules/no-shadow-restricted-names) |[noShadowRestrictedNames](/linter/rules//lint/rules/no-shadow-restricted-names) |
| [no-sparse-array](https://eslint.org/docs/latest/rules/no-sparse-array) |[noSparseArray](/linter/rules//lint/rules/no-sparse-array) |
| [no-this-before-super](https://eslint.org/docs/latest/rules/no-this-before-super) |[noUnreachableSuper](/linter/rules//lint/rules/no-unreachable-super) |
| [no-undef](https://eslint.org/docs/latest/rules/no-undef) |[noUndeclaredVariables](/linter/rules//lint/rules/no-undeclared-variables) |
| [no-unreachable](https://eslint.org/docs/latest/rules/no-unreachable) |[noUnreachable](/linter/rules//lint/rules/no-unreachable) |
| [no-unsafe-finally](https://eslint.org/docs/latest/rules/no-unsafe-finally) |[noUnsafeFinally](/linter/rules//lint/rules/no-unsafe-finally) |
| [no-unsafe-negation](https://eslint.org/docs/latest/rules/no-unsafe-negation) |[noUnsafeNegation](/linter/rules//lint/rules/no-unsafe-negation) |
| [no-unsafe-optional-chaining](https://eslint.org/docs/latest/rules/no-unsafe-optional-chaining) |[noUnsafeOptionalChaining](/linter/rules//lint/rules/no-unsafe-optional-chaining) |
| [no-unused-labels](https://eslint.org/docs/latest/rules/no-unused-labels) |[noUnusedLabels](/linter/rules//lint/rules/no-unused-labels) |
| [no-unused-vars](https://eslint.org/docs/latest/rules/no-unused-vars) |[noUnusedVariables](/linter/rules//lint/rules/no-unused-variables) |
| [no-useless-catch](https://eslint.org/docs/latest/rules/no-useless-catch) |[noUselessCatch](/linter/rules//lint/rules/no-useless-catch) |
| [no-useless-rename](https://eslint.org/docs/latest/rules/no-useless-rename) |[noUselessRename](/linter/rules//lint/rules/no-useless-rename) |
| [no-var](https://eslint.org/docs/latest/rules/no-var) |[noVar](/linter/rules//lint/rules/no-var) |
| [no-void](https://eslint.org/docs/latest/rules/no-void) |[noVoid](/linter/rules//lint/rules/no-void) |
| [no-with](https://eslint.org/docs/latest/rules/no-with) |[noWith](/linter/rules//lint/rules/no-with) |
| [one-var](https://eslint.org/docs/latest/rules/one-var) |[useSingleVarDeclarator](/linter/rules//lint/rules/use-single-var-declarator) |
| [operator-assignment](https://eslint.org/docs/latest/rules/operator-assignment) |[useShorthandAssign](/linter/rules//lint/rules/use-shorthand-assign) |
| [prefer-arrow-callback](https://eslint.org/docs/latest/rules/prefer-arrow-callback) |[useArrowFunction](/linter/rules//lint/rules/use-arrow-function) |
| [prefer-const](https://eslint.org/docs/latest/rules/prefer-const) |[useConst](/linter/rules//lint/rules/use-const) |
| [prefer-exponentiation-operator](https://eslint.org/docs/latest/rules/prefer-exponentiation-operator) |[useExponentiationOperator](/linter/rules//lint/rules/use-exponentiation-operator) |
| [prefer-numeric-literals](https://eslint.org/docs/latest/rules/prefer-numeric-literals) |[useNumericLiterals](/linter/rules//lint/rules/use-numeric-literals) |
| [prefer-regex-literals](https://eslint.org/docs/latest/rules/prefer-regex-literals) |[useRegexLiterals](/linter/rules//lint/rules/use-regex-literals) |
| [prefer-rest-params](https://eslint.org/docs/latest/rules/prefer-rest-params) |[noArguments](/linter/rules//lint/rules/no-arguments) |
| [prefer-template](https://eslint.org/docs/latest/rules/prefer-template) |[useTemplate](/linter/rules//lint/rules/use-template) |
| [require-yield](https://eslint.org/docs/latest/rules/require-yield) |[useYield](/linter/rules//lint/rules/use-yield) |
| [use-isnan](https://eslint.org/docs/latest/rules/use-isnan) |[useIsNan](/linter/rules//lint/rules/use-is-nan) |
| [valid-typeof](https://eslint.org/docs/latest/rules/valid-typeof) |[useValidTypeof](/linter/rules//lint/rules/use-valid-typeof) |
## eslint-plugin-import rules to Biome
| eslint-plugin-import rule name | Biome rule name |
| ---- | ---- |
| [no-default-export](https://github.com/import-js/eslint-plugin-import/blob/main/docs/rules/no-default-export.md) |[noDefaultExport](/linter/rules//lint/rules/no-default-export) |
## eslint-plugin-jsx-a11y rules to Biome
| eslint-plugin-jsx-a11y rule name | Biome rule name |
| ---- | ---- |
| [alt-text](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/alt-text.md) |[useAltText](/linter/rules//lint/rules/use-alt-text) |
| [anchor-has-content](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/anchor-has-content.md) |[useAnchorContent](/linter/rules//lint/rules/use-anchor-content) |
| [anchor-is-valid](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/anchor-is-valid.md) |[useValidAnchor](/linter/rules//lint/rules/use-valid-anchor) |
| [aria-activedescendant-has-tabindex](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-activedescendant-has-tabindex.md) |[useAriaActivedescendantWithTabindex](/linter/rules//lint/rules/use-aria-activedescendant-with-tabindex) |
| [aria-props](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-props.md) |[useValidAriaProps](/linter/rules//lint/rules/use-valid-aria-props) |
| [aria-proptypes](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-proptypes.md) |[useValidAriaValues](/linter/rules//lint/rules/use-valid-aria-values) |
| [aria-role](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-role.md) |[useValidAriaRole](/linter/rules//lint/rules/use-valid-aria-role) |
| [aria-unsupported-elements](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-unsupported-elements.md) |[noAriaUnsupportedElements](/linter/rules//lint/rules/no-aria-unsupported-elements) |
| [click-events-have-key-events](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/click-events-have-key-events.md) |[useKeyWithClickEvents](/linter/rules//lint/rules/use-key-with-click-events) |
| [heading-has-content](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/heading-has-content.md) |[useHeadingContent](/linter/rules//lint/rules/use-heading-content) |
| [html-has-lang](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/html-has-lang.md) |[useHtmlLang](/linter/rules//lint/rules/use-html-lang) |
| [iframe-has-title](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/iframe-has-title.md) |[useIframeTitle](/linter/rules//lint/rules/use-iframe-title) |
| [lang](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/lang.md) |[useValidLang](/linter/rules//lint/rules/use-valid-lang) |
| [media-has-caption](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/media-has-caption.md) |[useMediaCaption](/linter/rules//lint/rules/use-media-caption) |
| [mouse-events-have-key-events](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/mouse-events-have-key-events.md) |[useKeyWithMouseEvents](/linter/rules//lint/rules/use-key-with-mouse-events) |
| [no-access-key](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-access-key.md) |[noAccessKey](/linter/rules//lint/rules/no-access-key) |
| [no-aria-hidden-on-focusable](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-aria-hidden-on-focusable.md) |[noAriaHiddenOnFocusable](/linter/rules//lint/rules/no-aria-hidden-on-focusable) |
| [no-autofocus](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-autofocus.md) |[noAutofocus](/linter/rules//lint/rules/no-autofocus) |
| [no-distracting-elements](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-distracting-elements.md) |[noDistractingElements](/linter/rules//lint/rules/no-distracting-elements) |
| [no-interactive-element-to-noninteractive-role](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-interactive-element-to-noninteractive-role.md) |[noInteractiveElementToNoninteractiveRole](/linter/rules//lint/rules/no-interactive-element-to-noninteractive-role) |
| [no-noninteractive-element-to-interactive-role](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-noninteractive-element-to-interactive-role.md) |[noNoninteractiveElementToInteractiveRole](/linter/rules//lint/rules/no-noninteractive-element-to-interactive-role) |
| [no-noninteractive-tabindex](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-noninteractive-tabindex.md) |[noNoninteractiveTabindex](/linter/rules//lint/rules/no-noninteractive-tabindex) |
| [no-redundant-roles](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-redundant-roles.md) |[noRedundantAlt](/linter/rules//lint/rules/no-redundant-alt) |
| [role-has-required-aria-props](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/role-has-required-aria-props.md) |[useAriaPropsForRole](/linter/rules//lint/rules/use-aria-props-for-role) |
| [scope](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/scope.md) |[noHeaderScope](/linter/rules//lint/rules/no-header-scope) |
| [tabindex-no-positive](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/tabindex-no-positive.md) |[noPositiveTabindex](/linter/rules//lint/rules/no-positive-tabindex) |
## @mysticatea/eslint-plugin rules to Biome
| @mysticatea/eslint-plugin rule name | Biome rule name |
| ---- | ---- |
| [no-this-in-static](https://github.com/mysticatea/eslint-plugin/blob/master/docs/rules/no-this-in-static.md) |[noThisInStatic](/linter/rules//lint/rules/no-this-in-static) |
## eslint-plugin-react rules to Biome
| eslint-plugin-react rule name | Biome rule name |
| ---- | ---- |
| [button-has-type](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/button-has-type.md) |[useButtonType](/linter/rules//lint/rules/use-button-type) |
| [jsx-boolean-value](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-boolean-value.md) |[noImplicitBoolean](/linter/rules//lint/rules/no-implicit-boolean) |
| [jsx-fragments](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-fragments.md) |[useFragmentSyntax](/linter/rules//lint/rules/use-fragment-syntax) |
| [jsx-no-comment-textnodes](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-no-comment-textnodes.md) |[noCommentText](/linter/rules//lint/rules/no-comment-text) |
| [jsx-no-duplicate-props](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-no-duplicate-props.md) |[noDuplicateJsxProps](/linter/rules//lint/rules/no-duplicate-jsx-props) |
| [jsx-no-target-blank](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-no-target-blank.md) |[noBlankTarget](/linter/rules//lint/rules/no-blank-target) |
| [jsx-no-useless-fragment](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/jsx-no-useless-fragment.md) |[noUselessFragments](/linter/rules//lint/rules/no-useless-fragments) |
| [no-array-index-key](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/no-array-index-key.md) |[noArrayIndexKey](/linter/rules//lint/rules/no-array-index-key) |
| [no-children-prop](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/no-children-prop.md) |[noChildrenProp](/linter/rules//lint/rules/no-children-prop) |
| [no-danger](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/no-danger.md) |[noDangerouslySetInnerHtmlWithChildren](/linter/rules//lint/rules/no-dangerously-set-inner-html-with-children) |
| [no-danger-with-children](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/no-danger-with-children.md) |[noDangerouslySetInnerHtml](/linter/rules//lint/rules/no-dangerously-set-inner-html) |
| [void-dom-elements-no-children](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/void-dom-elements-no-children.md) |[noVoidElementsWithChildren](/linter/rules//lint/rules/no-void-elements-with-children) |
## eslint-plugin-react-hooks rules to Biome
| eslint-plugin-react-hooks rule name | Biome rule name |
| ---- | ---- |
| [exhaustive-deps](https://github.com/facebook/react/blob/main/packages/eslint-plugin-react-hooks/README.md) |[useExhaustiveDependencies](/linter/rules//lint/rules/use-exhaustive-dependencies) |
| [rules-of-hooks](https://github.com/facebook/react/blob/main/packages/eslint-plugin-react-hooks/README.md) |[useHookAtTopLevel](/linter/rules//lint/rules/use-hook-at-top-level) |
## eslint-plugin-sonarjs rules to Biome
| eslint-plugin-sonarjs rule name | Biome rule name |
| ---- | ---- |
| [cognitive-complexity](https://github.com/SonarSource/eslint-plugin-sonarjs/blob/HEAD/docs/rules/cognitive-complexity.md) |[noExcessiveCognitiveComplexity](/linter/rules//lint/rules/no-excessive-cognitive-complexity) |
## eslint-plugin-stylistic rules to Biome
| eslint-plugin-stylistic rule name | Biome rule name |
| ---- | ---- |
| [jsx-self-closing-comp](https://eslint.style/rules/default/jsx-self-closing-comp) |[useSelfClosingElements](/linter/rules//lint/rules/use-self-closing-elements) |
## eslint-plugin-typescript rules to Biome
| eslint-plugin-typescript rule name | Biome rule name |
| ---- | ---- |
| [array-type](https://typescript-eslint.io/rules/array-type) |[useShorthandArrayType](/linter/rules//lint/rules/use-shorthand-array-type) |
| [ban-types](https://typescript-eslint.io/rules/ban-types) |[noBannedTypes](/linter/rules//lint/rules/no-banned-types) |
| [naming-convention](https://typescript-eslint.io/rules/naming-convention) |[useNamingConvention](/linter/rules//lint/rules/use-naming-convention) |
| [no-empty-interface](https://typescript-eslint.io/rules/no-empty-interface) |[noEmptyInterface](/linter/rules//lint/rules/no-empty-interface) |
| [no-explicit-any](https://typescript-eslint.io/rules/no-explicit-any) |[noExplicitAny](/linter/rules//lint/rules/no-explicit-any) |
| [no-extra-non-null-assertion](https://typescript-eslint.io/rules/no-extra-non-null-assertion) |[noExtraNonNullAssertion](/linter/rules//lint/rules/no-extra-non-null-assertion) |
| [no-extraneous-class](https://typescript-eslint.io/rules/no-extraneous-class) |[noStaticOnlyClass](/linter/rules//lint/rules/no-static-only-class) |
| [no-inferrable-types](https://typescript-eslint.io/rules/no-inferrable-types) |[noInferrableTypes](/linter/rules//lint/rules/no-inferrable-types) |
| [no-invalid-void-type](https://typescript-eslint.io/rules/no-invalid-void-type) |[noConfusingVoidType](/linter/rules//lint/rules/no-confusing-void-type) |
| [no-misused-new](https://typescript-eslint.io/rules/no-misused-new) |[noMisleadingInstantiator](/linter/rules//lint/rules/no-misleading-instantiator) |
| [no-namespace](https://typescript-eslint.io/rules/no-namespace) |[noNamespace](/linter/rules//lint/rules/no-namespace) |
| [no-non-null-assertion](https://typescript-eslint.io/rules/no-non-null-assertion) |[noNonNullAssertion](/linter/rules//lint/rules/no-non-null-assertion) |
| [no-redeclare](https://typescript-eslint.io/rules/no-redeclare) |[noRedeclare](/linter/rules//lint/rules/no-redeclare) |
| [no-this-alias](https://typescript-eslint.io/rules/no-this-alias) |[noUselessThisAlias](/linter/rules//lint/rules/no-useless-this-alias) |
| [no-unnecessary-type-constraint](https://typescript-eslint.io/rules/no-unnecessary-type-constraint) |[noUselessTypeConstraint](/linter/rules//lint/rules/no-useless-type-constraint) |
| [no-unsafe-declaration-merging](https://typescript-eslint.io/rules/no-unsafe-declaration-merging) |[noUnsafeDeclarationMerging](/linter/rules//lint/rules/no-unsafe-declaration-merging) |
| [no-useless-constructor](https://typescript-eslint.io/rules/no-useless-constructor) |[noUselessConstructor](/linter/rules//lint/rules/no-useless-constructor) |
| [no-useless-empty-export](https://typescript-eslint.io/rules/no-useless-empty-export) |[noUselessEmptyExport](/linter/rules//lint/rules/no-useless-empty-export) |
| [no-useless-template-literals](https://typescript-eslint.io/rules/no-useless-template-literals) |[noUnusedTemplateLiteral](/linter/rules//lint/rules/no-unused-template-literal) |
| [parameter-properties](https://typescript-eslint.io/rules/parameter-properties) |[noParameterProperties](/linter/rules//lint/rules/no-parameter-properties) |
| [prefer-as-const](https://typescript-eslint.io/rules/prefer-as-const) |[useAsConstAssertion](/linter/rules//lint/rules/use-as-const-assertion) |
| [prefer-enum-initializers](https://typescript-eslint.io/rules/prefer-enum-initializers) |[useEnumInitializers](/linter/rules//lint/rules/use-enum-initializers) |
| [prefer-literal-enum-member](https://typescript-eslint.io/rules/prefer-literal-enum-member) |[useLiteralEnumMembers](/linter/rules//lint/rules/use-literal-enum-members) |
| [prefer-namespace-keyword](https://typescript-eslint.io/rules/prefer-namespace-keyword) |[useNamespaceKeyword](/linter/rules//lint/rules/use-namespace-keyword) |
| [prefer-optional-chain](https://typescript-eslint.io/rules/prefer-optional-chain) |[useOptionalChain](/linter/rules//lint/rules/use-optional-chain) |
## eslint-plugin-unicorn rules to Biome
| eslint-plugin-unicorn rule name | Biome rule name |
| ---- | ---- |
| [no-array-for-each](https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-array-for-each.md) |[noForEach](/linter/rules//lint/rules/no-for-each) |
| [no-instanceof-array](https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-instanceof-array.md) |[useIsArray](/linter/rules//lint/rules/use-is-array) |
| [no-useless-switch-case](https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-useless-switch-case.md) |[noUselessSwitchCase](/linter/rules//lint/rules/no-useless-switch-case) |
| [prefer-array-flat-map](https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/prefer-array-flat-map.md) |[useFlatMap](/linter/rules//lint/rules/use-flat-map) |
