use std::collections::BTreeMap;

use rustc_hash::{FxHashMap, FxHashSet};

use biome_analyze::{FixKind, RuleSource};
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::{
    CanBeImportedExported, ClosureExtensions, ReferencesExtensions, SemanticModel, is_constant,
};
use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsExpression,
    AnyJsMemberExpression, AnyJsObjectBindingPatternMember, JsArrayBindingPattern,
    JsArrayBindingPatternElement, JsArrayBindingPatternElementList, JsArrayExpression,
    JsComputedMemberExpression, JsObjectBindingPattern, JsObjectBindingPatternPropertyList,
    JsReferenceIdentifier, JsVariableDeclarator, JsxReferenceIdentifier, T, TsTypeofType,
    is_transparent_expression_wrapper,
};
use biome_js_syntax::{
    JsCallExpression, JsSyntaxKind, JsSyntaxNode, JsVariableDeclaration, TextRange,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, SyntaxNodeCast, TriviaPieceKind,
    declare_node_union,
};
use biome_rule_options::use_exhaustive_dependencies::{
    StableHookResult, UseExhaustiveDependenciesOptions,
};

/// Maximum recursion depth for stability checking to prevent stack overflow
const MAX_STABILITY_DEPTH: u8 = 10;

use crate::JsRuleAction;
use crate::react::hooks::*;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforce correct dependency usage within React hooks.
    ///
    /// React components have access to various [hooks](https://react.dev/reference/react/hooks) that can perform
    /// various actions like querying and updating state.
    ///
    /// For hooks that trigger whenever a variable changes (such as `useEffect` and `useMemo`),
    /// React relies on the hook's listed dependencies array to determine when to re-compute Effects and re-render the page.
    ///
    /// This can lead to unexpected behavior when dependencies are incorrectly specified:
    /// ```jsx,ignore
    ///
    /// function ticker() {
    ///   const [count, setCount] = useState(0);
    ///
    ///   /** Increment the count once per second. */
    ///   function onTick() {
    ///     setCount(count + 1);
    ///   }
    ///
    ///   // React _thinks_ this code doesn't depend on anything else, so
    ///   // it will only use the _initial_ version of `onTick` when rendering the component.
    ///   // As a result, our normally-dynamic counter will always display 1!
    ///   // This is referred to as a "stale closure", and is a common pitfall for beginners.
    ///   useEffect(() => {
    ///     const id = setInterval(onTick, 1000);
    ///     return () => clearInterval(id);
    ///   }, []);
    ///
    ///   return <h1>Counter: {count}</h1>;
    /// }
    /// ```
    ///
    /// ```jsx,ignore
    /// function apples() {
    ///   const [count, setCount] = useState(0);
    ///   const [message, setMessage] = useState("We have 0 apples!");
    ///
    ///   // React _thinks_ this code depends on BOTH `count` and `message`, and will re-run the hook whenever
    ///   // `message` is changed despite it not actually being used inside the closure.
    ///   // In fact, this will create an infinite loop due to our hook updating `message` and triggering itself again!
    ///   useEffect(() => {
    ///     setMessage(`We have ${count} apples!`)
    ///   }, [count, message]);
    ///
    /// }
    /// ```
    ///
    /// This rule attempts to prevent such issues by diagnosing potentially incorrect or invalid usages of hook dependencies.
    ///
    /// ### Default Behavior
    /// By default, the following hooks (and their Preact counterparts) will have their arguments checked by this rule:
    ///
    /// - `useEffect`
    /// - `useLayoutEffect`
    /// - `useInsertionEffect`
    /// - `useCallback`
    /// - `useMemo`
    /// - `useImperativeHandle`
    ///
    /// #### Stable results
    /// When a hook is known to have a stable return value (one whose identity doesn't change across invocations),
    /// that value doesn't need to and _should not_ be specified as a dependency.
    /// For example, setters returned by React's `useState` hook will not change throughout the lifetime of a program
    /// and should therefore be omitted.
    ///
    /// By default, the following hooks are considered to have stable return values:
    /// - `useState` (index 1)
    /// - `useReducer` (index 1)
    /// - `useTransition` (index 1)
    /// - `useRef`
    /// - `useEffectEvent`
    ///
    /// If you want to add custom hooks to the rule's diagnostics or specify your own functions with stable results,
    /// see the [options](#options) section for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///   let a = 1;
    ///   useEffect(() => {
    ///     console.log(a);
    ///   }, []);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function badComponent() {
    ///   let a = 1;
    ///   useEffect(() => {
    ///     console.log(a);
    ///   }, "not an array");
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///     let unused = 1;
    ///     useEffect(() => {}, [unused]);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect, useState } from "react";
    ///
    /// function component() {
    ///   const [name, setName] = useState();
    ///   useEffect(() => {
    ///     console.log(name);
    ///     setName("i never change and don't need to be here");
    ///   }, [name, setName]);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect, useState } from "react";
    ///
    /// function component() {
    ///   const name = "foo"
    ///   // name doesn't change, so specifying it is redundant
    ///   useEffect(() => {
    ///     console.log(name);
    ///   }, [name]);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///   let a = 1;
    ///   const b = a + 1;
    ///   useEffect(() => {
    ///     console.log(b);
    ///   }, []);
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// import { useCallback } from "react";
    ///
    /// function component() {
    ///   const Component = () => null;
    ///   const render = useCallback(() => <Component />, []);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///   let a = 1;
    ///   useEffect(() => {
    ///     console.log(a);
    ///   }, [a]);
    /// }
    /// ```
    ///
    /// ```js
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///   const SECONDS_PER_DAY = 60 * 60 * 24;
    ///   useEffect(() => {
    ///     console.log(SECONDS_PER_DAY);
    ///   });
    /// }
    /// ```
    ///
    /// ```js
    /// import { useEffect, useState } from "react";
    ///
    /// function component() {
    ///   const [name, setName] = useState();
    ///   useEffect(() => {
    ///     console.log(name);
    ///     setName("");
    ///   }, [name]);
    /// }
    /// ```
    ///
    /// Hooks not imported from React are ignored by default (unless specified inside [rule options](#options))
    /// ```ts
    /// import type { EffectCallback, DependencyList } from "react";
    /// // custom useEffect function
    /// declare function useEffect(cb: EffectCallback, deps?: DependencyList): void;
    ///
    /// function component() {
    ///   let name = "John Doe";
    ///   useEffect(() => {
    ///     console.log(name);
    ///   }, []);
    /// }
    /// ```
    ///
    /// ## Ignoring a specific dependency
    ///
    /// Sometimes you may wish to ignore a diagnostic about a specific
    /// dependency without disabling *all* linting for that hook. To do so,
    /// you may specify the name of a specific dependency between parentheses,
    /// like this:
    ///
    /// ```js
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///   let a = 1;
    ///   // biome-ignore lint/correctness/useExhaustiveDependencies(a): suppress dependency a
    ///   useEffect(() => {
    ///     console.log(a);
    ///   }, []);
    /// }
    /// ```
    ///
    /// If you wish to ignore multiple dependencies, you can add multiple
    /// comments and add a reason for each:
    ///
    /// ```js
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///   let a = 1;
    ///   let b = 1;
    ///   // biome-ignore lint/correctness/useExhaustiveDependencies(a): suppress dependency a
    ///   // biome-ignore lint/correctness/useExhaustiveDependencies(b): suppress dependency b
    ///   useEffect(() => {
    ///     console.log(a, b);
    ///   }, []);
    /// }
    /// ```
    ///
    /// :::caution
    /// Mismatching code & dependencies has a **very high risk** of creating bugs in your components.
    /// By suppressing the linter, you “lie” to React about the values your Effect depends on,
    /// so prefer changing the code over suppressing the rule where possible.
    /// :::
    ///
    /// ## Options
    ///
    /// ### `hooks`
    /// Allows specifying custom hooks (from libraries or internal projects) whose dependencies
    /// should be checked and/or which are known to have stable return values.
    ///
    /// For every hook whose dependencies you want validated, you must specify the index of both the closure
    /// using the dependencies and the dependencies array to validate it against.
    ///
    /// ##### Example
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "hooks": [
    ///       { "name": "useLocation", "closureIndex": 0, "dependenciesIndex": 1 },
    ///       { "name": "useQuery", "closureIndex": 2, "dependenciesIndex": 0 }
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// This would enable checks on the following code snippets:
    ///
    /// ```js,expect_diagnostic,use_options
    /// function Foo() {
    ///   let stateVar = 1;
    ///   useLocation(() => {console.log(stateVar)}, []);
    /// }
    /// ```
    /// ```js,use_options
    /// function Foo() {
    ///   let stateVar = 1;
    ///   useQuery([stateVar], "smthng", () => {console.log(stateVar)});
    /// }
    /// ```
    ///
    /// #### Configuring stable results
    ///
    /// As previously discussed, the lint rule takes into account so-called [stable results](#stable-results)
    /// and will ensure any such variables are _not_ specified as dependencies.
    ///
    /// You can specify custom functions as returning stable results in one of four ways:
    ///
    /// 1. `"stableResult": true` -- marks the return value as stable. An example
    ///   of a React hook that would be configured like this is `useRef()`.
    /// 2. `"stableResult": [1]` -- expects the return value to be an array and
    ///    marks the given indices as stable. An example of a React
    ///    hook that would be configured like this is `useState()`.
    /// 3. `"stableResult": 1` -- shorthand for option 2 (`"stableResult": [1]`).
    ///    Useful for hooks that only have a single stable return.
    /// 4. `"stableResult": ["setValue"]` -- expects the return value to be an
    ///    object and marks the properties with the given keys as stable.
    ///
    /// ##### Example
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "hooks": [
    ///       { "name": "useDispatch", "stableResult": true }
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// With this configuration, the following is valid:
    ///
    /// ```js,use_options
    /// const dispatch = useDispatch();
    /// // No need to list `dispatch` as dependency since it doesn't change
    /// const doAction = useCallback(() => dispatch(someAction()), []);
    /// ```
    ///
    /// ### `reportUnnecessaryDependencies`
    ///
    /// If set to `false`, the rule will not trigger diagnostics for unused dependencies passed to hooks that do not use them.
    ///
    /// :::caution
    /// Over-specifying dependencies can reduce application performance or even cause infinite loops, so caution is advised.
    /// :::
    ///
    /// Default: `true`
    ///
    /// ##### Example
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "reportUnnecessaryDependencies": false
    ///   }
    /// }
    /// ```
    ///
    /// ```jsx,use_options
    /// import { useEffect } from "react";
    ///
    /// function Foo() {
    ///   let stateVar = 1;
    ///   // not used but still OK
    ///   useEffect(() => {}, [stateVar]);
    /// }
    /// ```
    ///
    /// ### `reportMissingDependenciesArray`
    ///
    /// If enabled, the rule will also trigger diagnostics for hooks that lack dependency arrays altogether,
    /// requiring any hooks lacking dependencies to explicitly specify an empty array.
    ///
    /// Default: `false`
    ///
    /// ##### Example
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "reportMissingDependenciesArray": true
    ///   }
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// function noArrayYesProblem() {
    ///   let stateVar = 1;
    ///   React.useEffect(() => {});
    /// }
    /// ```
    ///
    pub UseExhaustiveDependencies {
        version: "1.0.0",
        name: "useExhaustiveDependencies",
        language: "jsx",
        sources: &[RuleSource::EslintReactHooks("exhaustive-deps").same()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::React, RuleDomain::Next],
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug, Clone)]
pub struct HookConfigMaps {
    pub(crate) hooks_config: FxHashMap<Box<str>, ReactHookConfiguration>,
    pub(crate) stable_config: FxHashSet<StableReactHookConfiguration>,
}

impl Default for HookConfigMaps {
    fn default() -> Self {
        let hooks_config: std::collections::HashMap<Box<str>, _, rustc_hash::FxBuildHasher> =
            FxHashMap::from_iter([
                ("useEffect".into(), (0, 1, true).into()),
                ("useLayoutEffect".into(), (0, 1, true).into()),
                ("useInsertionEffect".into(), (0, 1, true).into()),
                ("useCallback".into(), (0, 1, true).into()),
                ("useMemo".into(), (0, 1, true).into()),
                ("useImperativeHandle".into(), (1, 2, true).into()),
            ]);

        let stable_config = FxHashSet::from_iter([
            StableReactHookConfiguration::new("useState", StableHookResult::Indices(vec![1]), true),
            StableReactHookConfiguration::new(
                "useReducer",
                StableHookResult::Indices(vec![1]),
                true,
            ),
            StableReactHookConfiguration::new(
                "useTransition",
                StableHookResult::Indices(vec![1]),
                true,
            ),
            StableReactHookConfiguration::new("useRef", StableHookResult::Identity, true),
            StableReactHookConfiguration::new("useEffectEvent", StableHookResult::Identity, true),
        ]);

        Self {
            hooks_config,
            stable_config,
        }
    }
}

impl HookConfigMaps {
    pub fn new(hooks: &UseExhaustiveDependenciesOptions) -> Self {
        let mut result = Self::default();
        for hook in hooks.hooks.iter().flatten() {
            if let Some(stable_result) = &hook.stable_result
                && *stable_result != StableHookResult::None
            {
                result.stable_config.insert(StableReactHookConfiguration {
                    hook_name: hook.name.clone(),
                    result: stable_result.clone(),
                    builtin: false,
                });
            }
            if let (Some(closure_index), Some(dependencies_index)) =
                (hook.closure_index, hook.dependencies_index)
            {
                result.hooks_config.insert(
                    hook.name.clone(),
                    ReactHookConfiguration {
                        closure_index,
                        dependencies_index,
                        builtin: false,
                    },
                );
            }
        }

        result
    }
}

/// Flags the possible fixes that were found
pub enum Fix {
    /// When the entire dependencies array is missing
    MissingDependenciesArray { function_name_range: TextRange },
    /// When the dependency array is not an array literal node.
    NonLiteralDependenciesArray { expr: AnyJsExpression },
    /// When a dependency needs to be added.
    AddDependency {
        function_name_range: TextRange,
        captures: (Box<str>, Box<[JsSyntaxNode]>),
        dependencies_array: JsArrayExpression,
    },
    /// When a dependency needs to be removed.
    RemoveDependency {
        function_name_range: TextRange,
        component_function: JsSyntaxNode,
        dependencies: Box<[AnyJsExpression]>,
        dependencies_array: JsArrayExpression,
    },
    /// When a dependency is too unstable (changes every render).
    DependencyTooUnstable {
        dependency_name: Box<str>,
        dependency_range: TextRange,
        kind: UnstableDependencyKind,
    },
    /// When a dependency is more deep than the capture
    DependencyTooDeep {
        function_name_range: TextRange,
        capture_range: TextRange,
        dependency_range: TextRange,
        dependency_text: Box<str>,
    },
}

pub enum UnstableDependencyKind {
    Function,
    ObjectLiteral,
}

declare_node_union! {
    pub AnyExpressionCandidate = AnyJsExpression | JsReferenceIdentifier | JsxReferenceIdentifier
}

/// Returns expression candidates for a given reference for further checking.
/// The latest candidate is the longest member access chain.
/// Example: if the expression is `a.b[c]` it will return 3 candidates: `a`, `a.b`, `a.b[c]`
fn get_expression_candidates(node: JsSyntaxNode) -> Vec<AnyExpressionCandidate> {
    let mut result = Vec::new();

    if let Some(jsx_ref) = JsxReferenceIdentifier::cast_ref(&node) {
        result.push(AnyExpressionCandidate::JsxReferenceIdentifier(
            jsx_ref.clone(),
        ));
        return result;
    }

    let mut prev_node = node;
    while let Some(parent) = prev_node.parent() {
        if matches!(
            parent.kind(),
            JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER
        ) {
            if let Some(expression) = AnyExpressionCandidate::cast_ref(&prev_node) {
                result.push(expression.clone());
            }
            return result;
        }

        if matches!(parent.kind(), JsSyntaxKind::JS_SEQUENCE_EXPRESSION) {
            return result;
        }

        if is_transparent_expression_wrapper(&parent) {
            prev_node = parent;
            continue;
        }

        if let Some(computed_member_expression) = JsComputedMemberExpression::cast_ref(&parent)
            && let Ok(object) = computed_member_expression.object()
        {
            if !prev_node.eq(object.syntax()) {
                return result;
            }
            // collect only constant member access expressions
            if let Ok(member) = computed_member_expression.member()
                && !is_constant(&member)
            {
                return result;
            }
        }

        if matches!(
            parent.kind(),
            JsSyntaxKind::JS_IDENTIFIER_EXPRESSION
                | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
        ) {
            if let Some(sequence_expression) = AnyExpressionCandidate::cast_ref(&parent) {
                result.push(sequence_expression.clone());
            }
        } else {
            return result;
        }

        prev_node = parent;
    }

    result
}

// Test if a capture needs to be in the dependency list
// of a React hook call
fn capture_needs_to_be_in_the_dependency_list(
    capture_node: &JsSyntaxNode,
    expression_candidates: &[AnyExpressionCandidate],
    component_function_range: &TextRange,
    model: &SemanticModel,
    options: &HookConfigMaps,
) -> bool {
    // Ignore if referenced in TS typeof
    if capture_node
        .ancestors()
        .any(|a| TsTypeofType::can_cast(a.kind()))
    {
        return false;
    }

    if expression_candidates.is_empty() {
        return false;
    }

    !expression_candidates
        .iter()
        .any(|expression_candidate| match &expression_candidate {
            AnyExpressionCandidate::AnyJsExpression(expression) => is_stable_expression(
                expression,
                None,
                component_function_range,
                model,
                options,
                0,
            ),
            AnyExpressionCandidate::JsReferenceIdentifier(reference_identifier) => {
                if let Some(binding) = model.binding(reference_identifier) {
                    is_stable_binding(
                        &binding.tree(),
                        None,
                        component_function_range,
                        model,
                        options,
                        0,
                    )
                } else {
                    true
                }
            }
            AnyExpressionCandidate::JsxReferenceIdentifier(reference_identifier) => {
                if let Some(binding) = model.binding(reference_identifier) {
                    is_stable_binding(
                        &binding.tree(),
                        None,
                        component_function_range,
                        model,
                        options,
                        0,
                    )
                } else {
                    true
                }
            }
        })
}

/// Checks if a binding is stable within the component function.
///
/// Uses recursive calls to resolve variable references and member access.
/// The `depth` parameter prevents infinite recursion and stack overflow.
fn is_stable_binding(
    binding: &AnyJsIdentifierBinding,
    member: Option<&ReactHookResultMember>,
    component_function_range: &TextRange,
    model: &SemanticModel,
    options: &HookConfigMaps,
    depth: u8,
) -> bool {
    // Prevent excessive recursion by treating deeply nested checks as unstable
    if depth >= MAX_STABILITY_DEPTH {
        return false;
    }

    if binding.is_imported(model) {
        return true;
    }

    // Any declarations outside the component function are considered stable
    if binding
        .range()
        .intersect(*component_function_range)
        .is_none_or(TextRange::is_empty)
    {
        return true;
    }

    let Some(decl) = binding.declaration() else {
        return false;
    };

    match decl.parent_binding_pattern_declaration().unwrap_or(decl) {
        // These declarations are always stable
        AnyJsBindingDeclaration::JsClassDeclaration(_)
        | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::TsEnumDeclaration(_)
        | AnyJsBindingDeclaration::TsTypeAliasDeclaration(_)
        | AnyJsBindingDeclaration::TsInterfaceDeclaration(_)
        | AnyJsBindingDeclaration::TsExternalModuleDeclaration(_)
        | AnyJsBindingDeclaration::TsModuleDeclaration(_)
        | AnyJsBindingDeclaration::TsInferType(_)
        | AnyJsBindingDeclaration::TsMappedType(_)
        | AnyJsBindingDeclaration::TsTypeParameter(_)
        | AnyJsBindingDeclaration::TsEnumMember(_) => true,

        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            let Some(declaration) = declarator
                .syntax()
                .ancestors()
                .find_map(JsVariableDeclaration::cast)
            else {
                return true;
            };

            let Some(initializer_expression) = declarator
                .initializer()
                .and_then(|initializer| initializer.expression().ok())
            else {
                // No initializer - only `const` without initializer is stable
                // (this shouldn't happen for valid code, but handle gracefully)
                return declaration.is_const();
            };

            // For non-const declarations, only stable hook results (like useState setters
            // or useRef) are considered stable, AND only if the binding is never reassigned.
            if !declaration.is_const() {
                // First check if the binding was ever reassigned - if so, it's not stable
                if binding.all_writes(model).next().is_some() {
                    return false;
                }

                // Check if this is a stable hook result (e.g., setA from useState)
                return match get_single_pattern_member(binding, &declarator) {
                    GetSinglePatternMemberResult::Member(pattern_member) => {
                        if member.is_some() {
                            return false;
                        }
                        // Only check if initializer is a stable hook call
                        if let AnyJsExpression::JsCallExpression(call) = &initializer_expression {
                            is_react_hook_call_stable(
                                call,
                                Some(&pattern_member),
                                model,
                                &options.stable_config,
                            )
                        } else {
                            false
                        }
                    }
                    GetSinglePatternMemberResult::NoPattern => {
                        // For non-destructured let bindings like `let ref = useRef()`
                        if let AnyJsExpression::JsCallExpression(call) = &initializer_expression {
                            is_react_hook_call_stable(call, member, model, &options.stable_config)
                        } else {
                            false
                        }
                    }
                    _ => false,
                };
            }

            match get_single_pattern_member(binding, &declarator) {
                GetSinglePatternMemberResult::Member(pattern_member) => {
                    if member.is_some() {
                        // Too deeply nested
                        return false;
                    }
                    is_stable_expression(
                        &initializer_expression,
                        Some(&pattern_member),
                        component_function_range,
                        model,
                        options,
                        depth + 1,
                    )
                }
                GetSinglePatternMemberResult::NoPattern => is_stable_expression(
                    &initializer_expression,
                    member,
                    component_function_range,
                    model,
                    options,
                    depth + 1,
                ),
                GetSinglePatternMemberResult::TooDeep => false,
                GetSinglePatternMemberResult::Unknown => true,
            }
        }

        // all others need to be in the dependency list
        AnyJsBindingDeclaration::JsFunctionDeclaration(_)
        | AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
        | AnyJsBindingDeclaration::JsFormalParameter(_)
        | AnyJsBindingDeclaration::JsRestParameter(_)
        | AnyJsBindingDeclaration::JsBogusParameter(_)
        | AnyJsBindingDeclaration::TsIndexSignatureParameter(_)
        | AnyJsBindingDeclaration::TsPropertyParameter(_)
        | AnyJsBindingDeclaration::JsFunctionExpression(_)
        | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
        | AnyJsBindingDeclaration::JsClassExpression(_)
        | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::JsCatchDeclaration(_) => false,

        // Ignore TypeScript `import <id> =`
        AnyJsBindingDeclaration::TsImportEqualsDeclaration(_) => true,

        // This should be unreachable because we call `parent_binding_pattern_declaration`
        AnyJsBindingDeclaration::JsArrayBindingPatternElement(_)
        | AnyJsBindingDeclaration::JsArrayBindingPatternRestElement(_)
        | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_)
        | AnyJsBindingDeclaration::JsObjectBindingPatternRest(_)
        | AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_) => true,

        // This should be unreachable because of the test if the capture is imported
        AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => true,
    }
}

/// Checks if the expression is stable within the component function.
///
/// Uses recursive calls to resolve member access and variable references.
/// The `depth` parameter prevents infinite recursion and stack overflow.
fn is_stable_expression(
    expression: &AnyJsExpression,
    member: Option<&ReactHookResultMember>,
    component_function_range: &TextRange,
    model: &SemanticModel,
    options: &HookConfigMaps,
    depth: u8,
) -> bool {
    // Prevent excessive recursion by treating deeply nested checks as unstable
    if depth >= MAX_STABILITY_DEPTH {
        return false;
    }

    if model.is_constant(expression) {
        // If the expression creates a new object/function identity, it is not stable for React hooks,
        // even if it captures no variables.
        if !matches!(
            expression.syntax().kind(),
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_OBJECT_EXPRESSION
                | JsSyntaxKind::JS_ARRAY_EXPRESSION
                | JsSyntaxKind::JS_CLASS_EXPRESSION
                | JsSyntaxKind::JS_REGEX_LITERAL
        ) {
            return true;
        }
    }
    let Some(expression) = expression.inner_expression() else {
        return false;
    };
    match &expression {
        AnyJsExpression::JsCallExpression(call_expression) => {
            is_react_hook_call_stable(call_expression, member, model, &options.stable_config)
        }

        AnyJsExpression::JsComputedMemberExpression(computed_member_expression) => {
            // This rule handles only 1-level paths
            if member.is_some() {
                return false;
            }
            let (Ok(object), Some(index)) = (
                computed_member_expression.object(),
                computed_member_expression.member().ok().and_then(|member| {
                    member
                        .as_any_js_literal_expression()?
                        .as_js_number_literal_expression()?
                        .value_token()
                        .ok()?
                        .text_trimmed()
                        .parse::<u8>()
                        .ok()
                }),
            ) else {
                return false;
            };
            is_stable_expression(
                &object,
                Some(&ReactHookResultMember::Index(index)),
                component_function_range,
                model,
                options,
                depth + 1,
            )
        }
        AnyJsExpression::JsStaticMemberExpression(static_member_expression) => {
            // This rule handles only 1-level paths
            if member.is_some() {
                return false;
            }
            let (Ok(object), Some(key)) = (
                static_member_expression.object(),
                static_member_expression
                    .member()
                    .ok()
                    .and_then(|member| member.value_token().ok())
                    .map(|value_token| value_token.token_text_trimmed()),
            ) else {
                return false;
            };
            is_stable_expression(
                &object,
                Some(&ReactHookResultMember::Key(key)),
                component_function_range,
                model,
                options,
                depth + 1,
            )
        }

        AnyJsExpression::JsIdentifierExpression(identifier) => {
            if let Ok(name) = identifier.name()
                && let Some(binding) = model.binding(&name)
            {
                let binding = &binding.tree();
                let declaration = binding.declaration();
                let declaration_node = if let Some(
                    AnyJsBindingDeclaration::JsArrowFunctionExpression(arrow_function),
                ) = &declaration
                    && let Ok(AnyJsArrowFunctionParameters::AnyJsBinding(
                        AnyJsBinding::JsIdentifierBinding(identifier),
                    )) = arrow_function.parameters()
                    && identifier.syntax().eq(binding.syntax())
                {
                    Some(identifier.syntax().clone())
                } else {
                    declaration.map(|decl| decl.syntax().clone())
                };
                if let Some(declaration_node) = &declaration_node
                    && identifier
                        .syntax()
                        .ancestors()
                        .any(|ancestor| declaration_node == &ancestor)
                {
                    return true;
                }

                is_stable_binding(
                    binding,
                    member,
                    component_function_range,
                    model,
                    options,
                    depth + 1,
                )
            } else {
                true
            }
        }

        _ => false,
    }
}

/// Returns a single pattern member.
/// I.e., in the case of `const {x} = ...` returns `x`.
/// For deeply nested, i.e. `const {y: {x}} = ...` returns TooDeep.
/// For non-patterns returns NoPattern.
/// In case of any inconsistency returns Unknown.
fn get_single_pattern_member(
    binding: &AnyJsIdentifierBinding,
    declarator: &JsVariableDeclarator,
) -> GetSinglePatternMemberResult {
    let Some(parent_syntax) = binding.syntax().parent() else {
        return GetSinglePatternMemberResult::Unknown;
    };
    let Some((parent_pattern, member)) = (match parent_syntax.kind() {
        JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT => binding
            .parent::<JsArrayBindingPatternElement>()
            .and_then(|array_pattern_element| {
                array_pattern_element
                    .parent::<JsArrayBindingPatternElementList>()
                    .and_then(|array_pattern_element_list| {
                        array_pattern_element_list.parent::<JsArrayBindingPattern>()
                    })
                    .and_then(|array_pattern| {
                        (array_pattern_element.syntax().index() / 2)
                            .try_into()
                            .ok()
                            .map(|member| {
                                (
                                    array_pattern.syntax().clone(),
                                    Some(ReactHookResultMember::Index(member)),
                                )
                            })
                    })
            }),
        JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY
        | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
        | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST => {
            let Some(object_pattern) = parent_syntax
                .parent()
                .and_then(JsObjectBindingPatternPropertyList::cast)
                .and_then(|object_property_list| {
                    object_property_list.parent::<JsObjectBindingPattern>()
                })
            else {
                return GetSinglePatternMemberResult::Unknown;
            };
            if matches!(
                parent_syntax.kind(),
                JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST
            ) {
                Some((object_pattern.syntax().clone(), None))
            } else if let Some(member) =
                match AnyJsObjectBindingPatternMember::try_cast(parent_syntax) {
                    Ok(AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(
                        property,
                    )) => property
                        .member()
                        .ok()
                        .and_then(|member| member.name())
                        .map(ReactHookResultMember::Key),
                    Ok(
                        AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                            shorthand_property,
                        ),
                    ) => shorthand_property
                        .identifier()
                        .ok()
                        .and_then(|identifier| {
                            identifier.as_js_identifier_binding()?.name_token().ok()
                        })
                        .map(|name_token| {
                            ReactHookResultMember::Key(name_token.token_text_trimmed())
                        }),
                    // Shouldn't happen because of the previous check
                    _ => None,
                }
            {
                Some((object_pattern.syntax().clone(), Some(member)))
            } else {
                return GetSinglePatternMemberResult::Unknown;
            }
        }
        JsSyntaxKind::JS_VARIABLE_DECLARATOR => {
            return GetSinglePatternMemberResult::NoPattern;
        }
        _ => return GetSinglePatternMemberResult::Unknown,
    }) else {
        return GetSinglePatternMemberResult::Unknown;
    };
    if !parent_pattern
        .parent()
        .is_some_and(|syntax| syntax.eq(declarator.syntax()))
    {
        return GetSinglePatternMemberResult::TooDeep;
    }
    match member {
        Some(member) => GetSinglePatternMemberResult::Member(member),
        None => GetSinglePatternMemberResult::NoPattern,
    }
}

enum GetSinglePatternMemberResult {
    /// The binding is part of a pattern 1 level deep
    Member(ReactHookResultMember),
    /// The binding is not part of a pattern
    NoPattern,
    /// The binding is part of a too deeply nested pattern
    TooDeep,
    /// Could not determine
    Unknown,
}

// Find the function that is calling the hook
fn function_of_hook_call(call: &JsCallExpression) -> Option<JsSyntaxNode> {
    call.syntax().ancestors().find(|x| {
        matches!(
            x.kind(),
            JsSyntaxKind::JS_FUNCTION_DECLARATION
                | JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
                | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
        )
    })
}

/// Check if `identifier` is declared outside of `function` scope
fn is_out_of_function_scope(
    identifier: &AnyJsExpression,
    function: &JsSyntaxNode,
    model: &SemanticModel,
) -> Option<bool> {
    let identifier_name = identifier.as_js_identifier_expression()?.name().ok()?;

    let declaration = model.binding(&identifier_name)?.tree().declaration()?;

    Some(
        model
            .scope(declaration.syntax())
            .is_ancestor_of(&model.scope(function)),
    )
}

/// Checks if a dependency gets a new identity every render. If so, it returns
/// the kind of unstable dependency
///
/// Note we can only reliably determine this for some types of declarations.
/// Not every unstable dependency is expected to be reported.
///
/// This function assumes the identifier is declared within the same scope, so
/// it should only be used on dependencies that are otherwise correctly included
/// in the dependencies array.
fn determine_unstable_dependency(
    dependency: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<UnstableDependencyKind> {
    let identifier_name = dependency.as_js_identifier_expression()?.name().ok()?;

    let declaration = model.binding(&identifier_name)?.tree().declaration()?;
    match declaration {
        AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
        | AnyJsBindingDeclaration::JsFunctionDeclaration(_) => {
            Some(UnstableDependencyKind::Function)
        }
        AnyJsBindingDeclaration::JsArrayBindingPatternRestElement(_)
        | AnyJsBindingDeclaration::JsObjectBindingPatternRest(_) => {
            Some(UnstableDependencyKind::ObjectLiteral)
        }
        AnyJsBindingDeclaration::JsVariableDeclarator(declaration) => {
            let initializer = declaration.initializer()?;
            match initializer.expression().ok()? {
                AnyJsExpression::JsArrowFunctionExpression(_)
                | AnyJsExpression::JsFunctionExpression(_) => {
                    Some(UnstableDependencyKind::Function)
                }
                AnyJsExpression::JsArrayExpression(_) | AnyJsExpression::JsObjectExpression(_) => {
                    Some(UnstableDependencyKind::ObjectLiteral)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn into_member_iter(node: &JsSyntaxNode) -> impl Iterator<Item = String> + use<> {
    let mut vec = vec![];
    let mut next = Some(node.clone());

    while let Some(node) = next {
        match AnyJsMemberExpression::try_cast(node) {
            Ok(member_expr) => {
                let member_name = member_expr
                    .member_name()
                    .and_then(|it| it.as_string_constant().map(|it| it.to_owned()));
                if let Some(member_name) = member_name {
                    vec.push(member_name);
                }
                next = member_expr
                    .object()
                    .ok()
                    .and_then(|expr| expr.inner_expression())
                    .map(AstNode::into_syntax);
            }
            Err(node) => {
                vec.push(node.text_trimmed().to_string());
                break;
            }
        }
    }

    // elements are inserted in reverse, thus we have to reverse the iteration.
    vec.into_iter().rev()
}

fn compare_member_depth(a: &JsSyntaxNode, b: &JsSyntaxNode) -> (bool, bool) {
    let mut a_member_iter = into_member_iter(a);
    let mut b_member_iter = into_member_iter(b);

    loop {
        let a_member = a_member_iter.next();
        let b_member = b_member_iter.next();

        match (a_member, b_member) {
            (Some(a_member), Some(b_member)) => {
                if a_member != b_member {
                    return (false, false);
                }
            }
            (Some(_), None) => return (true, false),
            (None, Some(_)) => return (false, true),
            (None, None) => return (true, true),
        }
    }
}

/// Returns capture nodes for the given React hook call.
///
/// If the closure is an inline function expression, returns its captures.
/// If the closure is a function reference (identifier), resolves the function
/// and returns its captures. For identifiers that aren't functions (like props),
/// returns the identifier itself as it should be in the dependency list.
///
/// Bindings declared outside the component function range are treated as:
/// - Stable (ignored) if they're truly global or module-level
/// - Unstable (included) if they're function parameters (props)
fn get_relevant_capture_nodes(
    result: &ReactCallWithDependencyResult,
    model: &SemanticModel,
    component_function_range: &TextRange,
) -> Vec<JsSyntaxNode> {
    let Some(closure_expression) = result
        .closure_node
        .as_ref()
        .and_then(|node| node.inner_expression())
    else {
        return vec![];
    };

    if let AnyJsExpression::JsIdentifierExpression(identifier) = &closure_expression
        && let Ok(identifier_name) = identifier.name()
        && let Some(binding) = model.binding(&identifier_name)
        && let AnyJsIdentifierBinding::JsIdentifierBinding(identifier_binding) = binding.tree()
        && let Some(declaration) = identifier_binding.declaration()
    {
        if identifier_binding
            .range()
            .intersect(*component_function_range)
            .is_none_or(TextRange::is_empty)
        {
            return vec![];
        }

        let closure = match declaration {
            AnyJsBindingDeclaration::JsFunctionDeclaration(decl) => Some(decl.closure(model)),
            AnyJsBindingDeclaration::JsVariableDeclarator(decl) => decl
                .initializer()
                .and_then(|init| init.expression().ok())
                .and_then(|expr| AnyJsFunctionExpression::try_from(expr).ok())
                .map(|expr| expr.closure(model)),

            _ => None,
        };

        if let Some(new_closure) = closure {
            all_captures_in_closure(&new_closure)
                .map(|capture| capture.node().clone())
                .collect()
        } else {
            // Not a function - treat the identifier itself as a capture
            // (e.g., a prop that is a function)
            vec![identifier_name.syntax().clone()]
        }
    } else if let Ok(function_expression) = AnyJsFunctionExpression::try_from(closure_expression) {
        all_captures_in_closure(&function_expression.closure(model))
            .map(|capture| capture.node().clone())
            .collect()
    } else {
        vec![]
    }
}

impl Rule for UseExhaustiveDependencies {
    type Query = Semantic<JsCallExpression>;
    type State = Fix;
    type Signals = Box<[Self::State]>;
    type Options = UseExhaustiveDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let hook_config_maps = HookConfigMaps::new(options);

        let call = ctx.query();
        let model = ctx.model();

        let (Some(result), Some(component_function)) = (
            react_hook_with_dependency(call, &hook_config_maps.hooks_config, model),
            function_of_hook_call(call),
        ) else {
            return Vec::new().into_boxed_slice();
        };

        let mut signals = Vec::new();

        let dependencies_array = match &result.dependencies_node {
            Some(AnyJsExpression::JsArrayExpression(dependencies_array)) => dependencies_array,
            Some(expr) => {
                return vec![Fix::NonLiteralDependenciesArray { expr: expr.clone() }]
                    .into_boxed_slice();
            }
            None => {
                return if options.report_missing_dependencies_array() {
                    vec![Fix::MissingDependenciesArray {
                        function_name_range: result.function_name_range,
                    }]
                    .into_boxed_slice()
                } else {
                    Vec::new().into_boxed_slice()
                };
            }
        };

        let component_function_range = component_function.text_range_with_trivia();

        let captures: Vec<_> =
            get_relevant_capture_nodes(&result, model, &component_function_range)
                .iter()
                .filter_map(|capture_node| {
                    let expression_candidates = get_expression_candidates(capture_node.clone());
                    if capture_needs_to_be_in_the_dependency_list(
                        capture_node,
                        &expression_candidates,
                        &component_function_range,
                        model,
                        &hook_config_maps,
                    ) {
                        // Latest expression candidate is the longest expression
                        return Some(expression_candidates.last().map_or_else(
                            || capture_node.clone(),
                            |expression| expression.syntax().clone(),
                        ));
                    }
                    None
                })
                .collect();

        let deps: Vec<_> = result.all_dependencies().collect();
        let mut add_deps: BTreeMap<Box<str>, Vec<JsSyntaxNode>> = BTreeMap::new();

        // Evaluate all the captures
        for capture in &captures {
            let mut suggested_fix = None;
            let mut is_captured_covered = false;
            for dep in &deps {
                let (capture_contains_dep, dep_contains_capture) =
                    compare_member_depth(capture, dep.syntax());

                match (capture_contains_dep, dep_contains_capture) {
                    // capture == dependency
                    (true, true) => {
                        suggested_fix = None;
                        is_captured_covered = true;
                        break;
                    }
                    // example
                    // capture: a.b.c
                    // dependency: a
                    // this is ok, but we may suggest performance improvements
                    // in the future
                    (true, false) => {
                        // We need to continue, because it may still have a perfect match
                        // in the dependency list
                        is_captured_covered = true;
                    }
                    // example
                    // capture: a.b
                    // dependency: a.b.c
                    // This can be valid in some cases. We will flag an error nonetheless.
                    (false, true) => {
                        // We need to continue, because it may still have a perfect match
                        // in the dependency list
                        suggested_fix = Some(Fix::DependencyTooDeep {
                            function_name_range: result.function_name_range,
                            capture_range: capture.text_trimmed_range(),
                            dependency_range: dep.syntax().text_trimmed_range(),
                            dependency_text: dep.syntax().text_trimmed().into_text().into(),
                        });
                    }
                    _ => {}
                }
            }

            if let Some(fix) = suggested_fix {
                signals.push(fix);
            }

            if !is_captured_covered {
                let captures = add_deps
                    .entry(capture.text_trimmed().into_text().into())
                    .or_default();

                if !captures.iter().any(|existing| existing == capture) {
                    captures.push(capture.clone());
                }
            }
        }

        // Split deps into correctly specified ones and unnecessary ones.
        let (correct_deps, excessive_deps): (Vec<_>, Vec<_>) = deps.into_iter().partition(|dep| {
            captures.iter().any(|capture| {
                let (capture_contains_dep, dep_contains_capture) =
                    compare_member_depth(capture, dep.syntax());
                capture_contains_dep || dep_contains_capture
            })
        });

        // Find duplicated deps from specified ones
        {
            let mut dep_list: BTreeMap<String, AnyJsExpression> = BTreeMap::new();
            for dep in correct_deps.iter() {
                let expression_name = dep.to_string();
                if dep_list.contains_key(&expression_name) {
                    signals.push(Fix::RemoveDependency {
                        function_name_range: result.function_name_range,
                        component_function: component_function.clone(),
                        dependencies: vec![dep.clone()].into_boxed_slice(),
                        dependencies_array: dependencies_array.clone(),
                    });
                    continue;
                }
                dep_list.insert(expression_name, dep.clone());
            }
        }

        // Find correctly specified dependencies with an unstable identity,
        // since they would trigger re-evaluation on every render.
        let unstable_deps = correct_deps
            .into_iter()
            .filter_map(|dep| determine_unstable_dependency(&dep, model).map(|kind| (dep, kind)));

        // Generate signals
        for (name, nodes) in add_deps {
            signals.push(Fix::AddDependency {
                function_name_range: result.function_name_range,
                captures: (name, nodes.into_boxed_slice()),
                dependencies_array: dependencies_array.clone(),
            });
        }

        if options.report_unnecessary_dependencies() && !excessive_deps.is_empty() {
            signals.push(Fix::RemoveDependency {
                function_name_range: result.function_name_range,
                component_function,
                dependencies: excessive_deps.into_boxed_slice(),
                dependencies_array: dependencies_array.clone(),
            });
        }

        for (unstable_dep, kind) in unstable_deps {
            signals.push(Fix::DependencyTooUnstable {
                dependency_name: unstable_dep.syntax().to_string().into_boxed_str(),
                dependency_range: unstable_dep.range(),
                kind,
            });
        }

        signals.into_boxed_slice()
    }

    fn instances_for_signal(signal: &Self::State) -> Box<[Box<str>]> {
        match signal {
            Fix::MissingDependenciesArray { .. } => vec![].into_boxed_slice(),
            Fix::NonLiteralDependenciesArray { .. } => vec![].into_boxed_slice(),
            Fix::AddDependency { captures, .. } => vec![captures.0.clone()].into(),
            Fix::RemoveDependency { dependencies, .. } => dependencies
                .iter()
                .map(|dep| dep.syntax().text_trimmed().into_text().into())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            Fix::DependencyTooUnstable {
                dependency_name, ..
            } => vec![dependency_name.clone()].into(),
            Fix::DependencyTooDeep {
                dependency_text, ..
            } => vec![dependency_text.clone()].into(),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, dep: &Self::State) -> Option<RuleDiagnostic> {
        match dep {
            Fix::MissingDependenciesArray {
                function_name_range,
            } => Some(RuleDiagnostic::new(
                rule_category!(),
                function_name_range,
                markup! {"This hook does not have a dependencies array."},
            )
            .note(markup! {
                "React relies on hook dependencies to determine when to re-compute Effects."
                "\nAdd an explicit array (i.e. "<Emphasis>"[]"</Emphasis>") and list the callback's dependencies inside it."
                },
            )),
            Fix::NonLiteralDependenciesArray { expr } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    expr.range(),
                    markup! {"This dependencies list is not an array literal."},
                )
                .note(markup! {
                    "Biome can't statically verify whether you've passed the correct dependencies."
                    "\nReplace the argument with an array literal and list your dependencies within it."
                })
            ),
            Fix::AddDependency {
                function_name_range,
                captures,
                dependencies_array,
                ..
            } => {
                let (capture_text, captures_range) = captures;
                let mut diag = RuleDiagnostic::new(
                    rule_category!(),
                    function_name_range,
                    markup! {
                        "This hook "<Emphasis>"does not specify"</Emphasis>" its dependency on "<Emphasis>{capture_text.as_ref()}</Emphasis>"."
                    },
                ).note(markup! {
                    "React relies on hook dependencies to determine when to re-compute Effects."
                    "\nFailing to specify dependencies can result in Effects "<Emphasis>"not updating correctly"</Emphasis>" when state changes."
                    "\nThese \"stale closures\" are a common source of surprising bugs."
                    },
                );

                for range in captures_range {
                    diag = diag.detail(
                        range.text_trimmed_range(),
                        "This dependency is being used here, but is not specified in the hook dependency list.",
                    );
                }

                if dependencies_array.elements().len() == 0 {
                    diag = diag.note("Either include it or remove the dependency array.");
                }

                Some(diag)
            }
            Fix::RemoveDependency {
                function_name_range,
                dependencies,
                component_function,
                ..
            } => {
                let deps_joined_with_comma = dependencies
                    .iter()
                    .map(|dep| dep.syntax().text_trimmed().into_text().into())
                    .collect::<Vec<Box<str>>>()
                    .join(", ");
                let mut diag = RuleDiagnostic::new(
                    rule_category!(),
                    function_name_range,
                    markup! {
                        "This hook specifies "<Emphasis>"more dependencies than necessary"</Emphasis>": "{deps_joined_with_comma}"."
                    },
                )
                .note(markup! {
                        "React relies on hook dependencies to determine when to re-compute Effects."
                        "\nSpecifying more dependencies than required can lead to "<Emphasis>"unnecessary re-rendering"</Emphasis>
                        "\nand "<Emphasis>"degraded performance"</Emphasis>"."
                    },
                );

                let model = ctx.model();
                for dep in dependencies {
                    if is_out_of_function_scope(dep, component_function, model).unwrap_or(false) {
                        diag = diag.detail(
                            dep.syntax().text_trimmed_range(),
                            "Outer scope values aren't valid dependencies because mutating them doesn't re-render the component.",
                        );
                    } else {
                        diag = diag.detail(
                            dep.syntax().text_trimmed_range(),
                            "This dependency can be removed from the list.",
                        );
                    }
                }

                Some(diag)
            }
            Fix::DependencyTooUnstable {
                dependency_name,
                dependency_range,
                kind,
            } => {
                let suggested_hook = match kind {
                    UnstableDependencyKind::Function => "useCallback()",
                    UnstableDependencyKind::ObjectLiteral => "useMemo()",
                };
                let diag = RuleDiagnostic::new(
                    rule_category!(),
                    dependency_range,
                    markup! {
                        <Emphasis>{dependency_name.as_ref()}</Emphasis>" changes on every re-render and should not be used as a hook dependency."
                    },
                )
                .note(markup! {
                    "To fix this, wrap the definition of "<Emphasis>{dependency_name.as_ref()}</Emphasis>" in its own "<Emphasis>{suggested_hook}</Emphasis>" hook."
                });
                Some(diag)
            }
            Fix::DependencyTooDeep {
                function_name_range,
                capture_range,
                dependency_range,
                dependency_text,
            } => {
                let diag = RuleDiagnostic::new(
                    rule_category!(),
                    function_name_range,
                    markup! {
                        "This hook specifies a dependency more specific than its captures: "{dependency_text.as_ref()}""
                    },
                )
                .detail(capture_range, "This capture is more generic than...")
                .detail(dependency_range, "...this dependency.");
                Some(diag)
            }
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let message = match state {
            Fix::AddDependency {
                captures,
                dependencies_array,
                ..
            } => {
                let (capture_text, captures_range) = captures;
                let new_elements = captures_range.first().into_iter().filter_map(|node| {
                    if let Some(jsx_ref) = JsxReferenceIdentifier::cast_ref(node) {
                        return Some(AnyJsArrayElement::AnyJsExpression(
                            make::js_identifier_expression(make::js_reference_identifier(
                                jsx_ref.value_token().ok()?,
                            ))
                            .into(),
                        ));
                    }

                    node.ancestors()
                        .find_map(|node| match JsReferenceIdentifier::cast_ref(&node) {
                            Some(node) => Some(make::js_identifier_expression(node).into()),
                            _ => node.cast::<AnyJsExpression>(),
                        })
                        .and_then(|node| node.trim_trivia())
                        .map(AnyJsArrayElement::AnyJsExpression)
                });

                let elements = dependencies_array.elements();
                let elements = elements
                    .elements()
                    .flat_map(|element| element.into_node())
                    .chain(new_elements)
                    .collect::<Vec<_>>();

                mutation.replace_node(
                    dependencies_array.clone(),
                    recreate_array(dependencies_array, elements),
                );

                markup! { "Add the missing dependency "<Emphasis>{capture_text.as_ref()}</Emphasis>" to the list." }
            }
            Fix::RemoveDependency {
                dependencies,
                dependencies_array,
                ..
            } => {
                let elements = dependencies_array.elements();
                let elements = elements.elements()
                    .flat_map(|element| element.into_node())
                    .filter(|node| {
                        matches!(node, AnyJsArrayElement::AnyJsExpression(expr) if !dependencies.contains(expr))
                    })
                    .collect::<Vec<_>>();

                mutation.replace_node(
                    dependencies_array.clone(),
                    recreate_array(dependencies_array, elements),
                );

                markup! { "Remove the extra dependencies from the list." }
            }
            _ => return None,
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

fn recreate_array<E, I>(current: &JsArrayExpression, elements: E) -> JsArrayExpression
where
    E: IntoIterator<Item = AnyJsArrayElement, IntoIter = I>,
    I: ExactSizeIterator<Item = AnyJsArrayElement>,
{
    let elements = elements.into_iter();
    let separators = (0..elements.len().saturating_sub(1))
        .map(|_| make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]))
        .collect::<Vec<_>>();

    current
        .clone()
        .with_elements(make::js_array_element_list(elements, separators))
}
