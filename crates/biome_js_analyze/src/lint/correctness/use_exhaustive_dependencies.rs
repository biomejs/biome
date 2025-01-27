use crate::react::hooks::*;
use crate::services::semantic::Semantic;
use biome_analyze::RuleSource;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleDomain};
use biome_console::markup;
use biome_deserialize::{
    non_empty, DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::Deserializable;
use biome_js_semantic::{Capture, SemanticModel};
use biome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, JsCallExpression, JsSyntaxKind, JsSyntaxNode,
    JsVariableDeclaration, TextRange,
};
use biome_js_syntax::{AnyJsExpression, AnyJsMemberExpression, TsTypeofType};
use biome_rowan::{AstNode, SyntaxNodeCast};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use biome_diagnostics::Severity;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

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
    ///     // biome-ignore lint/correctness/useExhaustiveDependencies(a): <explanation>
    ///     useEffect(() => {
    ///         console.log(a);
    ///     }, []);
    /// }
    /// ```
    ///
    /// If you wish to ignore multiple dependencies, you can add multiple
    /// comments and add a reason for each.
    ///
    /// ## Options
    ///
    /// Allows to specify custom hooks - from libraries or internal projects -
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
    /// three ways:
    ///
    /// * `"stableResult": true` -- marks the return value as stable. An example
    ///   of a React hook that would be configured like this is `useRef()`.
    /// * `"stableResult": [1]` -- expects the return value to be an array and
    ///   marks the given index or indices to be stable. An example of a React
    ///   hook that would be configured like this is `useState()`.
    /// * `"stableResult": 1` -- shorthand for `"stableResult": [1]`.
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
        sources: &[RuleSource::EslintReactHooks("exhaustive-deps")],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::React, RuleDomain::Next],
    }
}

#[derive(Debug, Clone)]
pub struct HookConfigMaps {
    pub(crate) hooks_config: FxHashMap<String, ReactHookConfiguration>,
    pub(crate) stable_config: FxHashSet<StableReactHookConfiguration>,
}

impl Default for HookConfigMaps {
    fn default() -> Self {
        let hooks_config = FxHashMap::from_iter([
            ("useEffect".to_string(), (0, 1, true).into()),
            ("useLayoutEffect".to_string(), (0, 1, true).into()),
            ("useInsertionEffect".to_string(), (0, 1, true).into()),
            ("useCallback".to_string(), (0, 1, true).into()),
            ("useMemo".to_string(), (0, 1, true).into()),
            ("useImperativeHandle".to_string(), (1, 2, true).into()),
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
        ]);

        Self {
            hooks_config,
            stable_config,
        }
    }
}

/// Options for the rule `useExhaustiveDependencies`
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseExhaustiveDependenciesOptions {
    /// Whether to report an error when a dependency is listed in the dependencies array but isn't used. Defaults to true.
    #[serde(default = "report_unnecessary_dependencies_default")]
    pub report_unnecessary_dependencies: bool,

    /// Whether to report an error when a hook has no dependencies array.
    #[serde(default)]
    pub report_missing_dependencies_array: bool,

    /// List of hooks of which the dependencies should be validated.
    #[serde(default)]
    #[deserializable(validate = "non_empty")]
    pub hooks: Box<[Hook]>,
}

impl Default for UseExhaustiveDependenciesOptions {
    fn default() -> Self {
        Self {
            report_unnecessary_dependencies: report_unnecessary_dependencies_default(),
            report_missing_dependencies_array: false,
            hooks: Vec::new().into_boxed_slice(),
        }
    }
}

fn report_unnecessary_dependencies_default() -> bool {
    true
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct Hook {
    /// The name of the hook.
    #[deserializable(validate = "non_empty")]
    pub name: String,

    /// The "position" of the closure function, starting from zero.
    ///
    /// For example, for React's `useEffect()` hook, the closure index is 0.
    pub closure_index: Option<u8>,

    /// The "position" of the array of dependencies, starting from zero.
    ///
    /// For example, for React's `useEffect()` hook, the dependencies index is 1.
    pub dependencies_index: Option<u8>,

    /// Whether the result of the hook is stable.
    ///
    /// Set to `true` to mark the identity of the hook's return value as stable,
    /// or use a number/an array of numbers to mark the "positions" in the
    /// return array as stable.
    ///
    /// For example, for React's `useRef()` hook the value would be `true`,
    /// while for `useState()` it would be `[1]`.
    pub stable_result: Option<StableHookResult>,
}

impl DeserializableValidator for Hook {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: biome_rowan::TextRange,
    ) -> bool {
        match (self.closure_index, self.dependencies_index) {
            (Some(closure_index), Some(dependencies_index))
                if closure_index == dependencies_index =>
            {
                ctx.report(
                    DeserializationDiagnostic::new(markup! {
                        <Emphasis>"closureIndex"</Emphasis>" and "<Emphasis>"dependenciesIndex"</Emphasis>" may not be the same"
                    })
                    .with_range(range),
                );

                self.closure_index = None;
                self.dependencies_index = None;
            }
            _ => {}
        }

        true
    }
}

impl HookConfigMaps {
    pub fn new(hooks: &UseExhaustiveDependenciesOptions) -> Self {
        let mut result = HookConfigMaps::default();
        for hook in &hooks.hooks {
            if let Some(stable_result) = &hook.stable_result {
                if *stable_result != StableHookResult::None {
                    result.stable_config.insert(StableReactHookConfiguration {
                        hook_name: hook.name.clone(),
                        result: stable_result.clone(),
                        builtin: false,
                    });
                }
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
    /// When a dependency needs to be added.
    AddDependency {
        function_name_range: TextRange,
        captures: (Box<str>, Box<[TextRange]>),
        dependencies_len: usize,
    },
    /// When a dependency needs to be removed.
    RemoveDependency {
        function_name_range: TextRange,
        component_function: JsSyntaxNode,
        dependencies: Box<[AnyJsExpression]>,
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
                .map_or(true, TextRange::is_empty)
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
                .map_or(true, TextRange::is_empty)
            {
                return false;
            }

            if declaration.is_const() {
                // ... they are `const` and their initializer is constant
                if declarator
                    .initializer()
                    .and_then(|initializer| initializer.expression().ok())
                    .map_or(true, |expr| model.is_constant(&expr))
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

fn into_member_iter(node: &JsSyntaxNode) -> impl Iterator<Item = String> {
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

    // elemnsts are inserted in reverse, thus we have to reverse the iteration.
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
    type Options = Box<UseExhaustiveDependenciesOptions>;

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

            if result.dependencies_node.is_none() {
                if options.report_missing_dependencies_array {
                    return vec![Fix::MissingDependenciesArray {
                        function_name_range: result.function_name_range,
                    }]
                    .into_boxed_slice();
                } else {
                    return Vec::new().into_boxed_slice();
                }
            }

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
                    let path = get_whole_static_member_expression(capture.node());

                    match path {
                        Some(path) => (
                            path.syntax().text_trimmed().to_string(),
                            path.syntax().text_trimmed_range(),
                            path.syntax().clone(),
                        ),
                        None => (
                            capture.node().text_trimmed().to_string(),
                            capture.node().text_trimmed_range(),
                            capture.node().clone(),
                        ),
                    }
                })
                .collect();

            let deps: Vec<_> = result.all_dependencies().collect();
            let dependencies_len = deps.len();

            let mut add_deps: BTreeMap<Box<str>, Vec<TextRange>> = BTreeMap::new();

            // Evaluate all the captures
            for (capture_text, capture_range, capture_path) in captures.iter() {
                let mut suggested_fix = None;
                let mut is_captured_covered = false;
                for dep in deps.iter() {
                    let (capture_contains_dep, dep_contains_capture) =
                        compare_member_depth(capture_path, dep.syntax());

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
                                capture_range: *capture_range,
                                dependency_range: dep.syntax().text_trimmed_range(),
                                dependency_text: dep.syntax().text_trimmed().to_string().into(),
                            });
                        }
                        _ => {}
                    }
                }

                if let Some(fix) = suggested_fix {
                    signals.push(fix);
                }

                if !is_captured_covered {
                    let captures = add_deps.entry(capture_text.clone().into()).or_default();
                    captures.push(*capture_range);
                }
            }

            // Split deps into correctly specified ones and unnecessary ones.
            let (correct_deps, excessive_deps): (Vec<_>, Vec<_>) =
                deps.into_iter().partition(|dep| {
                    captures.iter().any(|(_, _, capture_path)| {
                        let (capture_contains_dep, dep_contains_capture) =
                            compare_member_depth(capture_path, dep.syntax());
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
            for (name, ranges) in add_deps {
                signals.push(Fix::AddDependency {
                    function_name_range: result.function_name_range,
                    captures: (name, ranges.into_boxed_slice()),
                    dependencies_len,
                });
            }

            if options.report_unnecessary_dependencies && !excessive_deps.is_empty() {
                signals.push(Fix::RemoveDependency {
                    function_name_range: result.function_name_range,
                    component_function,
                    dependencies: excessive_deps.into_boxed_slice(),
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
            Fix::MissingDependenciesArray {
                function_name_range: _,
            } => vec![].into_boxed_slice(),
            Fix::AddDependency { captures, .. } => vec![captures.0.clone()].into(),
            Fix::RemoveDependency { dependencies, .. } => dependencies
                .iter()
                .map(|dep| dep.syntax().text_trimmed().to_string().into_boxed_str())
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
            Fix::AddDependency {
                function_name_range,
                captures,
                dependencies_len,
            } => {
                let (capture_text, captures_range) = captures;
                let mut diag = RuleDiagnostic::new(
                    rule_category!(),
                    function_name_range,
                    markup! {"This hook does not specify all of its dependencies: "{capture_text.as_ref()}""},
                );

                for range in captures_range {
                    diag = diag.detail(
                        range,
                        "This dependency is not specified in the hook dependency list.",
                    );
                }

                if *dependencies_len == 0 {
                    diag = if captures_range.len() == 1 {
                        diag.note("Either include it or remove the dependency array")
                    } else {
                        diag.note("Either include them or remove the dependency array")
                    }
                }

                Some(diag)
            }
            Fix::RemoveDependency {
                function_name_range,
                dependencies,
                component_function,
            } => {
                let deps_joined_with_comma = dependencies
                    .iter()
                    .map(|dep| dep.syntax().text_trimmed().to_string())
                    .collect::<Vec<String>>()
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
}
