use crate::react::hooks::*;
use crate::semantic_services::Semantic;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_js_semantic::{Capture, SemanticModel};
use biome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, JsCallExpression, JsStaticMemberExpression, JsSyntaxKind,
    JsSyntaxNode, JsVariableDeclaration, TextRange,
};
use biome_json_syntax::{AnyJsonValue, JsonLanguage, JsonSyntaxNode};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNode, SyntaxNodeCast};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::str::FromStr;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_rule! {
    /// Enforce all dependencies are correctly specified.
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
    ///     });
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
    ///     });
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
    /// import { useEffect } from "react";
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
    pub(crate) hooks_config: HashMap<String, ReactHookConfiguration>,
    pub(crate) stable_config: HashSet<StableReactHookConfiguration>,
}

impl Default for ReactExtensiveDependenciesOptions {
    fn default() -> Self {
        let hooks_config = HashMap::from_iter([
            ("useEffect".to_string(), (0, 1).into()),
            ("useLayoutEffect".to_string(), (0, 1).into()),
            ("useInsertionEffect".to_string(), (0, 1).into()),
            ("useCallback".to_string(), (0, 1).into()),
            ("useMemo".to_string(), (0, 1).into()),
            ("useImperativeHandle".to_string(), (1, 2).into()),
            ("useState".to_string(), ReactHookConfiguration::default()),
            ("useContext".to_string(), ReactHookConfiguration::default()),
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
            ("useId".to_string(), ReactHookConfiguration::default()),
            (
                "useSyncExternalStore".to_string(),
                ReactHookConfiguration::default(),
            ),
        ]);

        let stable_config: HashSet<StableReactHookConfiguration> = HashSet::from_iter([
            StableReactHookConfiguration::new("useState", Some(1)),
            StableReactHookConfiguration::new("useReducer", Some(1)),
            StableReactHookConfiguration::new("useTransition", Some(1)),
            StableReactHookConfiguration::new("useRef", None),
            StableReactHookConfiguration::new("useContext", None),
            StableReactHookConfiguration::new("useId", None),
            StableReactHookConfiguration::new("useSyncExternalStore", None),
        ]);

        Self {
            hooks_config,
            stable_config,
        }
    }
}

/// Options for the rule `useExhaustiveDependencies` and `useHookAtTopLevel`
#[derive(Default, Deserialize, Serialize, Debug, Clone, Bpaf)]
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

#[derive(Default, Deserialize, Serialize, Debug, Clone, Bpaf)]
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

impl Hooks {
    const KNOWN_KEYS: &'static [&'static str] = &["name", "closureIndex", "dependenciesIndex"];
}

impl VisitNode<JsonLanguage> for Hooks {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, Hooks::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "name" => {
                self.name = self.map_to_string(&value, name_text, diagnostics)?;
            }
            "closureIndex" => {
                self.closure_index = self.map_to_usize(&value, name_text, usize::MAX, diagnostics);
            }
            "dependenciesIndex" => {
                self.dependencies_index =
                    self.map_to_usize(&value, name_text, usize::MAX, diagnostics);
            }
            _ => {}
        }

        Some(())
    }
}

impl FromStr for Hooks {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Hooks::default())
    }
}

impl VisitNode<JsonLanguage> for HooksOptions {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["hooks"], diagnostics)
    }

    fn visit_array_member(
        &mut self,
        element: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let mut hook = Hooks::default();
        let element = AnyJsonValue::cast_ref(element)?;
        self.map_to_object(&element, "hooks", &mut hook, diagnostics)?;
        if hook.name.is_empty() {
            diagnostics.push(
                DeserializationDiagnostic::new(markup!(
                    "The field "<Emphasis>"name"</Emphasis>" is mandatory"
                ))
                .with_range(element.range()),
            )
        } else {
            self.hooks.push(hook);
        }
        Some(())
    }

    fn visit_map(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        if name_text == "hooks" {
            let array = value.as_json_array_value()?;

            for element in array.elements() {
                let element = element.ok()?;
                let hook_array = element.as_json_array_value()?;

                let len = hook_array.elements().len();
                if len < 1 {
                    diagnostics.push(
                        DeserializationDiagnostic::new("At least one element is needed")
                            .with_range(hook_array.range()),
                    );
                    return Some(());
                }
                if len > 3 {
                    diagnostics.push(
                        DeserializationDiagnostic::new(
                            "Too many elements, maximum three are expected",
                        )
                        .with_range(hook_array.range()),
                    );
                    return Some(());
                }
                let mut elements = hook_array.elements().iter();
                let hook_name = elements.next()?.ok()?;
                let hook_name = hook_name
                    .as_json_string_value()
                    .ok_or_else(|| {
                        DeserializationDiagnostic::new_incorrect_type("string", hook_name.range())
                    })
                    .ok()?
                    .inner_string_text()
                    .ok()?
                    .to_string();

                let closure_index = if let Some(element) = elements.next() {
                    let element = element.ok()?;
                    Some(self.map_to_u8(&element, name_text, u8::MAX, diagnostics)? as usize)
                } else {
                    None
                };
                let dependencies_index = if let Some(element) = elements.next() {
                    let element = element.ok()?;
                    Some(self.map_to_u8(&element, name_text, u8::MAX, diagnostics)? as usize)
                } else {
                    None
                };

                self.hooks.push(Hooks {
                    name: hook_name,
                    closure_index,
                    dependencies_index,
                });
            }
        }
        Some(())
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
    AddDependency(TextRange, Vec<TextRange>),
    /// When a dependency needs to be removed.
    RemoveDependency(TextRange, Vec<TextRange>),
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
        | AnyJsBindingDeclaration::TsModuleDeclaration(_) => None,

        // Variable declarators are stable if ...
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            let declaration = declarator
                .syntax()
                .ancestors()
                .find_map(JsVariableDeclaration::cast)?;
            let declaration_range = declaration.syntax().text_range();

            if declaration.is_const() {
                // ... they are `const` and declared outside of the component function
                let _ = component_function_range.intersect(declaration_range)?;

                // ... they are `const` and their initializer is constant
                let initializer = declarator.initializer()?;
                let expr = initializer.expression().ok()?;
                if model.is_constant(&expr) {
                    return None;
                }
            }

            // ... they are assign to stable returns of another React function
            let not_stable = !is_binding_react_stable(&binding.tree(), &options.stable_config);
            not_stable.then_some(capture)
        }

        // all others need to be in the dependency list
        AnyJsBindingDeclaration::JsFormalParameter(_)
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

        // This should not be unreachable because of the test
        // if the capture is imported
        AnyJsBindingDeclaration::JsImportDefaultClause(_)
        | AnyJsBindingDeclaration::JsImportNamespaceClause(_)
        | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
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

            let deps: Vec<(String, TextRange)> = result
                .all_dependencies()
                .map(|dep| {
                    (
                        dep.syntax().text_trimmed().to_string(),
                        dep.syntax().text_trimmed_range(),
                    )
                })
                .collect();

            let mut add_deps: BTreeMap<String, Vec<TextRange>> = BTreeMap::new();
            let mut remove_deps: Vec<TextRange> = vec![];

            // Evaluate all the captures
            for (capture_text, capture_range, _) in captures.iter() {
                let mut suggested_fix = None;
                let mut is_captured_covered = false;
                for (dependency_text, dependency_range) in deps.iter() {
                    let capture_deeper_than_dependency = capture_text.starts_with(dependency_text);
                    let dependency_deeper_than_capture = dependency_text.starts_with(capture_text);
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
                                dependency_range: *dependency_range,
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

            // Search for dependencies not captured
            for (dependency_text, dep_range) in deps {
                let mut covers_any_capture = false;
                for (capture_text, _, _) in captures.iter() {
                    let capture_deeper_dependency = capture_text.starts_with(&dependency_text);
                    let dependency_deeper_capture = dependency_text.starts_with(capture_text);
                    if capture_deeper_dependency || dependency_deeper_capture {
                        covers_any_capture = true;
                        break;
                    }
                }

                if !covers_any_capture {
                    remove_deps.push(dep_range);
                }
            }

            // Generate signals
            for (_, captures) in add_deps {
                signals.push(Fix::AddDependency(result.function_name_range, captures));
            }

            if !remove_deps.is_empty() {
                signals.push(Fix::RemoveDependency(
                    result.function_name_range,
                    remove_deps,
                ));
            }
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, dep: &Self::State) -> Option<RuleDiagnostic> {
        match dep {
            Fix::AddDependency(use_effect_range, captures) => {
                let mut diag = RuleDiagnostic::new(
                    rule_category!(),
                    use_effect_range,
                    markup! {
                        "This hook do not specify all of its dependencies."
                    },
                );

                for range in captures.iter() {
                    diag = diag.detail(
                        range,
                        "This dependency is not specified in the hook dependency list.",
                    );
                }

                Some(diag)
            }
            Fix::RemoveDependency(use_effect_range, ranges) => {
                let mut diag = RuleDiagnostic::new(
                    rule_category!(),
                    use_effect_range,
                    markup! {
                        "This hook specifies more dependencies than necessary."
                    },
                );

                for range in ranges.iter() {
                    diag = diag.detail(range, "This dependency can be removed from the list.");
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
