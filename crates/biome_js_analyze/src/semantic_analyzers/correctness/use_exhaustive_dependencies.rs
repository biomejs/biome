use crate::react::hooks::*;
use crate::semantic_services::Semantic;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};
use biome_js_semantic::{Capture, SemanticModel};
use biome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, JsCallExpression, JsStaticMemberExpression, JsSyntaxKind,
    JsSyntaxNode, JsVariableDeclaration, TextRange,
};
use biome_js_syntax::{AnyJsExpression, JsIdentifierExpression, TsTypeofType};
use biome_rowan::{AstNode, SyntaxNodeCast};
use bpaf::Bpaf;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::str::FromStr;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_rule! {
    /// Enforce all dependencies are correctly specified in a React hook.
    ///
    /// This rule is a port of the rule [react-hooks/exhaustive-deps](https://legacy.reactjs.org/docs/hooks-rules.html#eslint-plugin), and it's meant to target projects that uses React.
    ///
    /// If your project _doesn't_ use React, **you shouldn't use this rule**.
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
    /// If you want to add more hooks to the rule, check the [#options](options).
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
    /// ## Valid
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
    /// ## Options
    ///
    /// Allows to specify custom hooks - from libraries or internal projects - that can be considered stable.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "hooks": [
    ///             { "name": "useLocation", "closureIndex": 0, "dependenciesIndex": 1},
    ///             { "name": "useQuery", "closureIndex": 1, "dependenciesIndex": 0}
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Given the previous example, your hooks be used like this:
    ///
    /// ```js
    /// function Foo() {
    ///     const location = useLocation(() => {}, []);
    ///     const query = useQuery([], () => {});
    /// }
    /// ```
    ///
    pub(crate) UseExhaustiveDependencies {
        version: "1.0.0",
        name: "useExhaustiveDependencies",
        recommended: true,
    }
}

#[derive(Debug, Clone)]
pub struct ReactExtensiveDependenciesOptions {
    pub(crate) hooks_config: FxHashMap<String, ReactHookConfiguration>,
    pub(crate) stable_config: FxHashSet<StableReactHookConfiguration>,
}

impl Default for ReactExtensiveDependenciesOptions {
    fn default() -> Self {
        let hooks_config = FxHashMap::from_iter([
            ("useEffect".to_string(), (0, 1).into()),
            ("useLayoutEffect".to_string(), (0, 1).into()),
            ("useInsertionEffect".to_string(), (0, 1).into()),
            ("useCallback".to_string(), (0, 1).into()),
            ("useMemo".to_string(), (0, 1).into()),
            ("useImperativeHandle".to_string(), (1, 2).into()),
            ("useState".to_string(), ReactHookConfiguration::default()),
            ("useReducer".to_string(), ReactHookConfiguration::default()),
            ("useRef".to_string(), ReactHookConfiguration::default()),
            (
                "useDebugValue".to_string(),
                ReactHookConfiguration::default(),
            ),
            (
                "useDeferredValue".to_string(),
                ReactHookConfiguration::default(),
            ),
            (
                "useTransition".to_string(),
                ReactHookConfiguration::default(),
            ),
        ]);

        let stable_config = FxHashSet::from_iter([
            StableReactHookConfiguration::new("useState", Some(1)),
            StableReactHookConfiguration::new("useReducer", Some(1)),
            StableReactHookConfiguration::new("useTransition", Some(1)),
            StableReactHookConfiguration::new("useRef", None),
        ]);

        Self {
            hooks_config,
            stable_config,
        }
    }
}

/// Options for the rule `useExhaustiveDependencies` and `useHookAtTopLevel`
#[derive(Default, Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HooksOptions {
    #[bpaf(external, hide, many)]
    /// List of safe hooks
    pub hooks: Vec<Hooks>,
}

impl FromStr for HooksOptions {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(HooksOptions::default())
    }
}

impl Deserializable for HooksOptions {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(HooksOptionsVisitor, name, diagnostics)
    }
}

struct HooksOptionsVisitor;
impl DeserializationVisitor for HooksOptionsVisitor {
    type Output = HooksOptions;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["hooks"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "hooks" => {
                    let val_range = value.range();
                    result.hooks = Deserializable::deserialize(&value, &key_text, diagnostics)
                        .unwrap_or_default();
                    if result.hooks.is_empty() {
                        diagnostics.push(
                            DeserializationDiagnostic::new("At least one element is needed")
                                .with_range(val_range),
                        );
                    }
                }
                text => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    text,
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}

#[derive(Default, Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Hooks {
    #[bpaf(hide)]
    /// The name of the hook
    pub name: String,
    #[bpaf(hide)]
    /// The "position" of the closure function, starting from zero.
    ///
    /// ### Example
    pub closure_index: Option<usize>,
    #[bpaf(hide)]
    /// The "position" of the array of dependencies, starting from zero.
    pub dependencies_index: Option<usize>,
}

impl FromStr for Hooks {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Hooks::default())
    }
}

impl Deserializable for Hooks {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(HooksVisitor, name, diagnostics)
    }
}

struct HooksVisitor;
impl DeserializationVisitor for HooksVisitor {
    type Output = Hooks;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["name", "closureIndex", "dependenciesIndex"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "name" => {
                    let val_range = value.range();
                    result.name = Deserializable::deserialize(&value, &key_text, diagnostics)
                        .unwrap_or_default();
                    if result.name.is_empty() {
                        diagnostics.push(
                            DeserializationDiagnostic::new(markup!(
                                "The field "<Emphasis>"name"</Emphasis>" is mandatory"
                            ))
                            .with_range(val_range),
                        )
                    }
                }
                "closureIndex" => {
                    result.closure_index =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "dependenciesIndex" => {
                    result.dependencies_index =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}

impl ReactExtensiveDependenciesOptions {
    pub fn new(hooks: HooksOptions) -> Self {
        let mut default = ReactExtensiveDependenciesOptions::default();
        for hook in hooks.hooks {
            default.hooks_config.insert(
                hook.name,
                ReactHookConfiguration {
                    closure_index: hook.closure_index,
                    dependencies_index: hook.dependencies_index,
                },
            );
        }

        default
    }
}

/// Flags the possible fixes that were found
pub enum Fix {
    /// When a dependency needs to be added.
    AddDependency {
        function_name_range: TextRange,
        captures: Vec<TextRange>,
        dependencies_len: usize,
    },
    /// When a dependency needs to be removed.
    RemoveDependency {
        function_name_range: TextRange,
        component_function: JsSyntaxNode,
        dependencies: Vec<AnyJsExpression>,
    },
    /// When a dependency is more deep than the capture
    DependencyTooDeep {
        function_name_range: TextRange,
        capture_range: TextRange,
        dependency_range: TextRange,
    },
}

fn get_whole_static_member_expression(
    reference: &JsSyntaxNode,
) -> Option<JsStaticMemberExpression> {
    let root = reference
        .ancestors()
        .skip(2) //IDENT and JS_REFERENCE_IDENTIFIER
        .take_while(|x| x.kind() == JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION)
        .last()?;
    root.cast()
}

// Test if a capture needs to be in the dependency list
// of a react hook call
fn capture_needs_to_be_in_the_dependency_list(
    capture: Capture,
    component_function_range: &TextRange,
    model: &SemanticModel,
    options: &ReactExtensiveDependenciesOptions,
) -> Option<Capture> {
    // Ignore if referenced in TS typeof
    if capture
        .node()
        .ancestors()
        .any(|a| TsTypeofType::can_cast(a.kind()))
    {
        return None;
    }

    let binding = capture.binding();

    // Ignore if imported
    if binding.is_imported() {
        return None;
    }

    match binding.tree().declaration()? {
        // These declarations are always stable
        AnyJsBindingDeclaration::JsFunctionDeclaration(_)
        | AnyJsBindingDeclaration::JsClassDeclaration(_)
        | AnyJsBindingDeclaration::TsEnumDeclaration(_)
        | AnyJsBindingDeclaration::TsTypeAliasDeclaration(_)
        | AnyJsBindingDeclaration::TsInterfaceDeclaration(_)
        | AnyJsBindingDeclaration::TsModuleDeclaration(_)
        | AnyJsBindingDeclaration::TsInferType(_)
        | AnyJsBindingDeclaration::TsMappedType(_)
        | AnyJsBindingDeclaration::TsTypeParameter(_) => None,
        // Variable declarators are stable if ...
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            let declaration = declarator
                .syntax()
                .ancestors()
                .find_map(JsVariableDeclaration::cast)?;
            let declaration_range = declaration.syntax().text_range();

            // ... they are declared outside of the component function
            let _ = component_function_range.intersect(declaration_range)?;

            if declaration.is_const() {
                // ... they are `const` and their initializer is constant
                let initializer = declarator.initializer()?;
                let expr = initializer.expression().ok()?;
                if model.is_constant(&expr) {
                    return None;
                }
            }

            // ... they are assign to stable returns of another React function
            let not_stable =
                !is_binding_react_stable(&binding.tree(), model, &options.stable_config);
            not_stable.then_some(capture)
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
        | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::JsCatchDeclaration(_) => Some(capture),

        // This should be unreachable because of the test if the capture is imported
        AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
        | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
        | AnyJsBindingDeclaration::TsImportEqualsDeclaration(_) => {
            unreachable!()
        }
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
    let identifier_name = JsIdentifierExpression::cast_ref(identifier.syntax())?
        .name()
        .ok()?;

    let declaration = model.binding(&identifier_name)?.tree().declaration()?;

    Some(
        model
            .scope(declaration.syntax())
            .is_ancestor_of(&model.scope(function)),
    )
}

impl Rule for UseExhaustiveDependencies {
    type Query = Semantic<JsCallExpression>;
    type State = Fix;
    type Signals = Vec<Self::State>;
    type Options = HooksOptions;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let options = ctx.options();
        let options = ReactExtensiveDependenciesOptions::new(options.clone());

        let mut signals = vec![];

        let call = ctx.query();
        let model = ctx.model();

        if let Some(result) = react_hook_with_dependency(call, &options.hooks_config, model) {
            let Some(component_function) = function_of_hook_call(call) else {
                return vec![];
            };

            if result.dependencies_node.is_none() {
                return vec![];
            }

            let component_function_range = component_function.text_range();

            let captures: Vec<_> = result
                .all_captures(model)
                .filter_map(|capture| {
                    capture_needs_to_be_in_the_dependency_list(
                        capture,
                        &component_function_range,
                        model,
                        &options,
                    )
                })
                .map(|capture| {
                    let path = get_whole_static_member_expression(capture.node());

                    let (text, range) = if let Some(path) = path {
                        (
                            path.syntax().text_trimmed().to_string(),
                            path.syntax().text_trimmed_range(),
                        )
                    } else {
                        (
                            capture.node().text_trimmed().to_string(),
                            capture.node().text_trimmed_range(),
                        )
                    };

                    (text, range, capture)
                })
                .collect();

            let deps: Vec<_> = result.all_dependencies().collect();
            let dependencies_len = deps.len();

            let mut add_deps: BTreeMap<String, Vec<TextRange>> = BTreeMap::new();

            // Evaluate all the captures
            for (capture_text, capture_range, _) in captures.iter() {
                let mut suggested_fix = None;
                let mut is_captured_covered = false;
                for dep in deps.iter() {
                    // capture_text and dependency_text should filter the "?" inside
                    // in order to ignore optional chaining
                    let filter_capture_text = capture_text.replace('?', "");
                    let filter_dependency_text =
                        dep.syntax().text_trimmed().to_string().replace('?', "");
                    let capture_deeper_than_dependency =
                        filter_capture_text.starts_with(&filter_dependency_text);
                    let dependency_deeper_than_capture =
                        filter_dependency_text.starts_with(&filter_capture_text);

                    match (
                        capture_deeper_than_dependency,
                        dependency_deeper_than_capture,
                    ) {
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
                            });
                        }
                        _ => {}
                    }
                }

                if let Some(fix) = suggested_fix {
                    signals.push(fix);
                }

                if !is_captured_covered {
                    let captures = add_deps.entry(capture_text.clone()).or_default();
                    captures.push(*capture_range);
                }
            }

            let mut remove_deps: Vec<AnyJsExpression> = vec![];
            // Search for dependencies not captured
            for dep in deps {
                let mut covers_any_capture = false;
                for (capture_text, _, _) in captures.iter() {
                    // capture_text and dependency_text should filter the "?" inside
                    // in order to ignore optional chaining
                    let filter_capture_text = capture_text.replace('?', "");
                    let filter_dependency_text =
                        dep.syntax().text_trimmed().to_string().replace('?', "");
                    let capture_deeper_dependency =
                        filter_capture_text.starts_with(&filter_dependency_text);
                    let dependency_deeper_capture =
                        filter_dependency_text.starts_with(&filter_capture_text);
                    if capture_deeper_dependency || dependency_deeper_capture {
                        covers_any_capture = true;
                        break;
                    }
                }

                if !covers_any_capture {
                    remove_deps.push(dep);
                }
            }

            // Generate signals
            for (_, captures) in add_deps {
                signals.push(Fix::AddDependency {
                    function_name_range: result.function_name_range,
                    captures,
                    dependencies_len,
                });
            }

            if !remove_deps.is_empty() {
                signals.push(Fix::RemoveDependency {
                    function_name_range: result.function_name_range,
                    component_function,
                    dependencies: remove_deps,
                });
            }
        }

        signals
    }

    fn diagnostic(ctx: &RuleContext<Self>, dep: &Self::State) -> Option<RuleDiagnostic> {
        match dep {
            Fix::AddDependency {
                function_name_range,
                captures,
                dependencies_len,
            } => {
                let mut diag = RuleDiagnostic::new(
                    rule_category!(),
                    function_name_range,
                    markup! {
                        "This hook does not specify all of its dependencies."
                    },
                );

                for range in captures {
                    diag = diag.detail(
                        range,
                        "This dependency is not specified in the hook dependency list.",
                    );
                }

                if *dependencies_len == 0 {
                    diag = if captures.len() == 1 {
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
                let mut diag = RuleDiagnostic::new(
                    rule_category!(),
                    function_name_range,
                    markup! {
                        "This hook specifies more dependencies than necessary."
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
            Fix::DependencyTooDeep {
                function_name_range,
                capture_range,
                dependency_range,
            } => {
                let diag = RuleDiagnostic::new(
                    rule_category!(),
                    function_name_range,
                    markup! {
                        "This hook specifies a dependency more specific that its captures"
                    },
                )
                .detail(capture_range, "This capture is more generic than...")
                .detail(dependency_range, "...this dependency.");
                Some(diag)
            }
        }
    }
}
