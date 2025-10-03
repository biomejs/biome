use std::collections::BTreeMap;

use rustc_hash::{FxHashMap, FxHashSet};

use biome_analyze::{FixKind, RuleSource};
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::{Capture, SemanticModel};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsMemberExpression, JsArrayExpression,
    JsReferenceIdentifier, T, TsTypeofType,
};
use biome_js_syntax::{
    JsCallExpression, JsSyntaxKind, JsSyntaxNode, JsVariableDeclaration, TextRange,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxNodeCast, TriviaPieceKind};
use biome_rule_options::use_exhaustive_dependencies::{
    StableHookResult, UseExhaustiveDependenciesOptions,
};

use crate::JsRuleAction;
use crate::react::hooks::*;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforce all dependencies are correctly specified in a React hook.
    ///
    /// _This rule should be used only in **React** projects._
    ///
    /// This rule is a port of the rule [react-hooks/exhaustive-deps](https://legacy.reactjs.org/docs/hooks-rules.html#eslint-plugin), and it's meant to target projects that uses React.
    ///
    /// If your project _doesn't_ use React (or Preact), **you shouldn't use this rule**.
    ///
    /// The rule will inspect the following **known** hooks:
    ///
    /// - `useEffect`
    /// - `useLayoutEffect`
    /// - `useInsertionEffect`
    /// - `useCallback`
    /// - `useMemo`
    /// - `useImperativeHandle`
    /// - `useState`
    /// - `useReducer`
    /// - `useRef`
    /// - `useDebugValue`
    /// - `useDeferredValue`
    /// - `useTransition`
    /// - `useEffectEvent`
    ///
    /// If you want to add more hooks to the rule, check the [options](#options).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///     let a = 1;
    ///     useEffect(() => {
    ///         console.log(a);
    ///     }, []);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///     let b = 1;
    ///     useEffect(() => {
    ///     }, [b]);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect, useState } from "react";
    ///
    /// function component() {
    ///     const [name, setName] = useState();
    ///     useEffect(() => {
    ///         console.log(name);
    ///         setName("");
    ///     }, [name, setName]);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///     let a = 1;
    ///     const b = a + 1;
    ///     useEffect(() => {
    ///         console.log(b);
    ///     }, []);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///     let a = 1;
    ///     useEffect(() => {
    ///         console.log(a);
    ///     }, [a]);
    /// }
    /// ```
    ///
    /// ```js
    /// import { useEffect } from "react";
    ///
    /// function component() {
    ///     const a = 1;
    ///     useEffect(() => {
    ///         console.log(a);
    ///     });
    /// }
    /// ```
    ///
    /// ```js
    /// import { useEffect, useState } from "react";
    ///
    /// function component() {
    ///     const [name, setName] = useState();
    ///     useEffect(() => {
    ///         console.log(name);
    ///         setName("");
    ///     }, [name]);
    /// }
    /// ```
    ///
    /// ```js
    /// import { useEffect } from "react";
    /// let outer = false;
    /// function component() {
    ///     useEffect(() => {
    ///         outer = true;
    ///     }, []);
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
    ///     let a = 1;
    ///     // biome-ignore lint/correctness/useExhaustiveDependencies(a): suppress dependency a
    ///     useEffect(() => {
    ///         console.log(a);
    ///     }, []);
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
    ///     let a = 1;
    ///     let b = 1;
    ///     // biome-ignore lint/correctness/useExhaustiveDependencies(a): suppress dependency a
    ///     // biome-ignore lint/correctness/useExhaustiveDependencies(b): suppress dependency b
    ///     useEffect(() => {
    ///         console.log(a, b);
    ///     }, []);
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// Allows specifying custom hooks - from libraries or internal projects -
    /// for which dependencies should be checked and/or which are known to have
    /// stable return values.
    ///
    /// ### Validating dependencies
    ///
    /// For every hook for which you want the dependencies to be validated, you
    /// should specify the index of the closure and the index of the
    /// dependencies array to validate against.
    ///
    /// #### Example
    ///
    /// ```json, options
    /// {
    ///     "options": {
    ///         "hooks": [
    ///             { "name": "useLocation", "closureIndex": 0, "dependenciesIndex": 1},
    ///             { "name": "useQuery", "closureIndex": 1, "dependenciesIndex": 0}
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Given the previous example, your hooks can be used like this:
    ///
    /// ```js
    /// function Foo() {
    ///     const location = useLocation(() => {}, []);
    ///     const query = useQuery([], () => {});
    /// }
    /// ```
    ///
    /// ### Stable results
    ///
    /// When a hook is known to have a stable return value (its identity doesn't
    /// change across invocations), that value doesn't need to be specified in
    /// dependency arrays. For example, setters returned by React's `useState`
    /// hook always have the same identity and should be omitted as such.
    ///
    /// You can configure custom hooks that return stable results in one of
    /// four ways:
    ///
    /// * `"stableResult": true` -- marks the return value as stable. An example
    ///   of a React hook that would be configured like this is `useRef()`.
    /// * `"stableResult": [1]` -- expects the return value to be an array and
    ///   marks the given index or indices to be stable. An example of a React
    ///   hook that would be configured like this is `useState()`.
    /// * `"stableResult": 1` -- shorthand for `"stableResult": [1]`.
    /// * `"stableResult": ["setValue"]` -- expects the return value to be an
    ///   object and marks the given property or properties to be stable.
    ///
    /// #### Example
    ///
    /// ```json
    /// {
    ///     "options": {
    ///         "hooks": [
    ///             { "name": "useDispatch", "stableResult": true }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// With this configuration, the following is valid:
    ///
    /// ```js
    /// const dispatch = useDispatch();
    /// // No need to list `dispatch` as dependency:
    /// const doAction = useCallback(() => dispatch(someAction()), []);
    /// ```
    ///
    /// ## Preact support
    ///
    /// This rule recognizes rules imported from `preact/compat` and
    /// `preact/hooks` and applies the same rules as for React hooks.
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
        for hook in &hooks.hooks {
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

fn get_whole_static_member_expression(reference: &JsSyntaxNode) -> Option<AnyJsMemberExpression> {
    let root = reference
        .ancestors()
        .skip(1) // JS_REFERENCE_IDENTIFIER
        .take_while(|x| {
            x.parent().is_some_and(|parent| {
                parent
                    .cast::<AnyJsMemberExpression>()
                    .is_some_and(|member_expr| {
                        member_expr
                            .object()
                            .is_ok_and(|object| object.syntax() == x)
                    })
            })
        })
        .last()?
        .parent()?;

    root.cast()
}

// Test if a capture needs to be in the dependency list
// of a react hook call
fn capture_needs_to_be_in_the_dependency_list(
    capture: &Capture,
    component_function_range: &TextRange,
    model: &SemanticModel,
    options: &HookConfigMaps,
) -> bool {
    // Ignore if referenced in TS typeof
    if capture
        .node()
        .ancestors()
        .any(|a| TsTypeofType::can_cast(a.kind()))
    {
        return false;
    }

    let binding = capture.binding();

    // Ignore if imported
    if binding.is_imported() {
        return false;
    }
    let Some(decl) = binding.tree().declaration() else {
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
        | AnyJsBindingDeclaration::TsEnumMember(_) => false,
        // Function declarations are stable if ...
        AnyJsBindingDeclaration::JsFunctionDeclaration(declaration) => {
            let declaration_range = declaration.syntax().text_range_with_trivia();

            // ... they are declared outside of the component function
            if component_function_range
                .intersect(declaration_range)
                .is_none_or(TextRange::is_empty)
            {
                return false;
            }

            // ... they are recursively used by the binding being created:
            //
            // function MyRecursiveElement() {
            // 	 const children = useMemo(() => <MyRecursiveElement />, []);
            // 	 return <div>{children}</div>;
            // }
            //
            if capture
                .node()
                .ancestors()
                .any(|ancestor| &ancestor == declaration.syntax())
            {
                return false;
            }

            true
        }
        // Variable declarators are stable if ...
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            let Some(declaration) = declarator
                .syntax()
                .ancestors()
                .find_map(JsVariableDeclaration::cast)
            else {
                return false;
            };
            let declaration_range = declaration.syntax().text_range_with_trivia();

            // ... they are declared outside of the component function
            if component_function_range
                .intersect(declaration_range)
                .is_none_or(TextRange::is_empty)
            {
                return false;
            }

            if declaration.is_const() {
                // ... they are `const` and their initializer is constant
                if declarator
                    .initializer()
                    .and_then(|initializer| initializer.expression().ok())
                    .is_none_or(|expr| model.is_constant(&expr))
                {
                    return false;
                }
            }

            // ... they are recursively used by the binding being created
            if capture
                .node()
                .ancestors()
                .any(|ancestor| &ancestor == declaration.syntax())
            {
                return false;
            }

            // ... they are assign to stable returns of another React function
            !is_binding_react_stable(&binding.tree(), model, &options.stable_config)
        }

        // all others need to be in the dependency list
        AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
        | AnyJsBindingDeclaration::JsFormalParameter(_)
        | AnyJsBindingDeclaration::JsRestParameter(_)
        | AnyJsBindingDeclaration::JsBogusParameter(_)
        | AnyJsBindingDeclaration::TsIndexSignatureParameter(_)
        | AnyJsBindingDeclaration::TsPropertyParameter(_)
        | AnyJsBindingDeclaration::JsFunctionExpression(_)
        | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
        | AnyJsBindingDeclaration::JsClassExpression(_)
        | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::JsCatchDeclaration(_) => true,

        // Ignore TypeScript `import <id> =`
        AnyJsBindingDeclaration::TsImportEqualsDeclaration(_) => false,

        // This should be unreachable because we call `parent_binding_pattern_declaration`
        AnyJsBindingDeclaration::JsArrayBindingPatternElement(_)
        | AnyJsBindingDeclaration::JsArrayBindingPatternRestElement(_)
        | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_)
        | AnyJsBindingDeclaration::JsObjectBindingPatternRest(_)
        | AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_) => false,

        // This should be unreachable because of the test if the capture is imported
        AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => false,
    }
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
                next = member_expr.object().ok().map(AstNode::into_syntax);
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

impl Rule for UseExhaustiveDependencies {
    type Query = Semantic<JsCallExpression>;
    type State = Fix;
    type Signals = Box<[Self::State]>;
    type Options = UseExhaustiveDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let hook_config_maps = HookConfigMaps::new(options);

        let mut signals = Vec::new();

        let call = ctx.query();
        let model = ctx.model();

        if let Some(result) =
            react_hook_with_dependency(call, &hook_config_maps.hooks_config, model)
        {
            let Some(component_function) = function_of_hook_call(call) else {
                return Vec::new().into_boxed_slice();
            };

            let dependencies_array = match &result.dependencies_node {
                Some(AnyJsExpression::JsArrayExpression(dependencies_array)) => dependencies_array,
                Some(expr) => {
                    return vec![Fix::NonLiteralDependenciesArray { expr: expr.clone() }]
                        .into_boxed_slice();
                }
                None => {
                    return if options.report_missing_dependencies_array {
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

            let captures: Vec<_> = result
                .all_captures(model)
                .filter(|capture| {
                    capture_needs_to_be_in_the_dependency_list(
                        capture,
                        &component_function_range,
                        model,
                        &hook_config_maps,
                    )
                })
                .map(|capture| {
                    get_whole_static_member_expression(capture.node())
                        .map_or_else(|| capture.node().clone(), |path| path.syntax().clone())
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
            let (correct_deps, excessive_deps): (Vec<_>, Vec<_>) =
                deps.into_iter().partition(|dep| {
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
            let unstable_deps = correct_deps.into_iter().filter_map(|dep| {
                determine_unstable_dependency(&dep, model).map(|kind| (dep, kind))
            });

            // Generate signals
            for (name, nodes) in add_deps {
                signals.push(Fix::AddDependency {
                    function_name_range: result.function_name_range,
                    captures: (name, nodes.into_boxed_slice()),
                    dependencies_array: dependencies_array.clone(),
                });
            }

            if options.report_unnecessary_dependencies && !excessive_deps.is_empty() {
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
                markup! {"This hook does not have a dependencies array"},
            )),
            Fix::NonLiteralDependenciesArray { expr } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    expr.range(),
                    markup! {"This dependencies list is not an array literal."},
                )
                .note(markup! {"Biome can't statically verify whether you've passed the correct dependencies."})
                .note(markup! { "Replace with an array literal and list your dependencies within it."})
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
                    markup! {"This hook does not specify its dependency on "<Emphasis>{capture_text.as_ref()}</Emphasis>"."},
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
                        "This hook specifies more dependencies than necessary: "{deps_joined_with_comma}""
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
                        "This hook specifies a dependency more specific that its captures: "{dependency_text.as_ref()}""
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
                captures: (_, captures),
                dependencies_array,
                ..
            } => {
                let new_elements = captures.first().into_iter().filter_map(|node| {
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

                markup! { "Add the missing dependency to the list." }
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
