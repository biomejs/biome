use crate::frameworks::vue::vue_component::{AnyPotentialVueComponent, VueComponentQuery};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBindingPattern, AnyJsExpression, AnyJsFunction,
    AnyJsObjectMember, AnyJsObjectMemberName, JsAssignmentExpression, JsCallExpression,
    JsExportDefaultExpressionClause, JsIdentifierBinding, JsImport, JsLanguage,
    JsMethodObjectMember, JsNamedImportSpecifier, JsObjectMemberList, JsParameters,
    JsReferenceIdentifier, JsShorthandNamedImportSpecifier, JsSyntaxKind, JsVariableDeclarator,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNode, TextRange};

declare_lint_rule! {
    /// Disallow destructuring of `props` passed to `setup` in Vue projects.
    ///
    /// In Vue's Composition API, props must be accessed as `props.propertyName` to maintain
    /// reactivity. Destructuring `props` in the root scope of the `setup` function will
    /// cause the resulting variables to lose their reactive nature.
    ///
    /// This rule flags destructuring in two cases:
    /// 1. Directly in the `setup` function's parameters.
    /// 2. In the root scope of the `setup` function's body.
    ///
    /// Destructuring is permitted inside nested functions or callbacks (e.g., inside `watch` or
    /// a returned render function), as the values are accessed at execution time when the
    /// reactive context is preserved. Using `toRefs` or `toRef` to create reactive `ref`s
    /// from props is also a valid and safe alternative.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// // Destructuring in setup parameters
    /// export default {
    ///   setup({ count }) {
    ///     // `count` is now a plain value, not a reactive property.
    ///     return () => h('div', count);
    ///   }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // Destructuring in the root scope of the setup function
    /// export default {
    ///   setup(props) {
    ///     const { count } = props;
    ///     // `count` has lost its reactivity.
    ///     return () => h('div', count);
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Accessing props directly preserves reactivity.
    /// export default {
    ///   setup(props) {
    ///     watch(() => props.count, () => { /* ... */ });
    ///     return () => h('div', props.count);
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// // Using `toRefs` converts props into a set of reactive refs.
    /// import { toRefs } from 'vue';
    ///
    /// export default {
    ///   setup(props) {
    ///     const { count } = toRefs(props); // OK: `count` is a ref.
    ///     return () => h('div', count.value);
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// // Destructuring is safe inside nested functions and callbacks.
    /// export default {
    ///   setup(props) {
    ///     return () => {
    ///       const { count } = props; // OK: evaluated during render.
    ///       return h('div', count);
    ///     }
    ///   }
    /// }
    /// ```
    ///
    pub NoVueSetupPropsReactivityLoss {
        version: "next",
        name: "noVueSetupPropsReactivityLoss",
        language: "js",
        domains: &[RuleDomain::Vue],
        recommended: false,
        sources: &[RuleSource::EslintVueJs("no-setup-props-reactivity-loss").inspired()],
    }
}

/// Represents the specific reason why the rule was triggered.
#[derive(Debug)]
pub enum Violation {
    /// `props` were destructured directly in the function's parameters.
    ParameterDestructuring(TextRange),
    /// `props` were destructured in the root scope of the `setup` function body.
    RootScopeDestructuring {
        destructuring_range: TextRange,
        props_param_range: TextRange,
    },
}

/// An abstraction to handle both function expressions and method declarations for `setup`.
#[derive(Debug)]
enum SetupFunction {
    Function(AnyJsFunction),
    Method(JsMethodObjectMember),
}

/// Contains information about a destructuring operation.
///
/// Used to check if props destructuring is safe (from `toRefs(props)`) or unsafe (from `props`).
#[derive(Debug)]
struct DestructuringInfo {
    destructuring_range: TextRange,
    /// The expression being destructured (e.g., `props` or `toRefs(props)`).
    initializer: Option<AnyJsExpression>,
}

impl Rule for NoVueSetupPropsReactivityLoss {
    type Query = VueComponentQuery;
    type State = Violation;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        // AnyPotentialVueComponent allows us to analyze different component patterns
        // (e.g., `export default {}`, `defineComponent({})`) in a unified way.
        analyze_vue_component(ctx.query(), ctx.model())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            Violation::ParameterDestructuring(range) => RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "Destructuring `props` in the `setup` function parameters loses reactivity."
                },
            ),
            Violation::RootScopeDestructuring {
                destructuring_range,
                props_param_range,
            } => RuleDiagnostic::new(
                rule_category!(),
                *destructuring_range,
                markup! {
                    "Destructuring `props` in the root scope of `setup` loses reactivity."
                },
            )
            .detail(
                *props_param_range,
                markup! {
                    "The `props` parameter is defined here."
                },
            ),
        };

        Some(
            diagnostic
                .note(markup! {
                    "To preserve reactivity, access props as properties: `props.propertyName`."
                })
                .note(markup! {
                    "Alternatively, use `toRefs(props)` or `toRef(props, 'key')` to create reactive refs."
                }),
        )
    }
}

fn analyze_vue_component(
    potential_component: &AnyPotentialVueComponent,
    model: &SemanticModel,
) -> Vec<Violation> {
    match potential_component {
        AnyPotentialVueComponent::JsExportDefaultExpressionClause(export_clause) => {
            extract_setup_from_export_default(export_clause, model)
        }
        AnyPotentialVueComponent::JsCallExpression(call_expr) => {
            extract_setup_from_call_expression(call_expr, model)
        }
    }
}

fn extract_setup_from_export_default(
    export_clause: &JsExportDefaultExpressionClause,
    model: &SemanticModel,
) -> Vec<Violation> {
    export_clause
        .expression()
        .ok()
        .and_then(|expr| {
            expr.as_js_object_expression()
                .map(|obj_expr| extract_setup_violations_from_members(&obj_expr.members(), model))
        })
        .unwrap_or_default()
}

fn extract_setup_from_call_expression(
    call_expr: &JsCallExpression,
    model: &SemanticModel,
) -> Vec<Violation> {
    call_expr
        .arguments()
        .ok()
        .and_then(|args| args.args().iter().next().and_then(|arg| arg.ok()))
        .and_then(|first_arg| {
            first_arg
                .as_any_js_expression()
                .and_then(|e| e.as_js_object_expression())
                .map(|obj_expr| extract_setup_violations_from_members(&obj_expr.members(), model))
        })
        .unwrap_or_default()
}

fn extract_setup_violations_from_members(
    members: &JsObjectMemberList,
    model: &SemanticModel,
) -> Vec<Violation> {
    members
        .iter()
        .filter_map(|member| member.ok())
        .filter_map(|member| extract_setup_from_object_member(&member))
        .flat_map(|setup_fn| check_setup_function(&setup_fn, model))
        .collect()
}

fn check_setup_function(setup_fn: &SetupFunction, model: &SemanticModel) -> Vec<Violation> {
    let Some(first_param) = extract_setup_first_parameter(setup_fn) else {
        return vec![];
    };

    match first_param {
        // Case: `setup({ count })` - parameter destructuring.
        AnyJsBindingPattern::JsObjectBindingPattern(obj_pattern) => {
            vec![Violation::ParameterDestructuring(obj_pattern.range())]
        }
        AnyJsBindingPattern::JsArrayBindingPattern(array_pattern) => {
            vec![Violation::ParameterDestructuring(array_pattern.range())]
        }
        // Case: `setup(props)` - find destructuring in the function body.
        AnyJsBindingPattern::AnyJsBinding(binding) => binding
            .as_js_identifier_binding()
            .map(|props_binding| {
                find_root_scope_destructuring_violations(setup_fn, props_binding, model)
            })
            .unwrap_or_default(),
    }
}

fn find_root_scope_destructuring_violations(
    setup_fn: &SetupFunction,
    props_binding: &JsIdentifierBinding,
    model: &SemanticModel,
) -> Vec<Violation> {
    let mut violations = Vec::new();

    for reference in model.as_binding(props_binding).all_reads() {
        if let Some(identifier) = JsReferenceIdentifier::cast_ref(reference.syntax())
            // Rule 1: Only flag destructuring in the root scope. Nested scopes are safe.
            && is_reference_in_root_scope_of_function(&identifier, setup_fn)
            // Check if this reference is being destructured.
            && let Some(destructuring_info) = is_reference_in_destructuring(&identifier)
            // Rule 2: Allow destructuring from reactive APIs like `toRefs(props)`.
            && !is_safe_reactive_destructuring(&destructuring_info, model)
        {
            violations.push(Violation::RootScopeDestructuring {
                destructuring_range: destructuring_info.destructuring_range,
                props_param_range: props_binding.range(),
            });
        }
    }

    violations
}

fn extract_setup_from_object_member(member: &AnyJsObjectMember) -> Option<SetupFunction> {
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => method
            .name()
            .ok()
            .filter(is_member_named_setup)
            .map(|_| SetupFunction::Method(method.clone())),
        AnyJsObjectMember::JsPropertyObjectMember(property) => property
            .name()
            .ok()
            .filter(is_member_named_setup)
            .and_then(|_| property.value().ok())
            .and_then(|value| extract_function_from_expression(&value))
            .map(SetupFunction::Function),
        _ => None,
    }
}

fn extract_setup_first_parameter(func: &SetupFunction) -> Option<AnyJsBindingPattern> {
    match func {
        SetupFunction::Function(any_func) => extract_js_function_first_parameter(any_func),
        SetupFunction::Method(method) => method
            .parameters()
            .ok()
            .as_ref()
            .and_then(get_first_parameter_binding),
    }
}

fn extract_js_function_first_parameter(func: &AnyJsFunction) -> Option<AnyJsBindingPattern> {
    match func {
        AnyJsFunction::JsFunctionDeclaration(decl) => decl
            .parameters()
            .ok()
            .as_ref()
            .and_then(get_first_parameter_binding),
        AnyJsFunction::JsFunctionExpression(expr) => expr
            .parameters()
            .ok()
            .as_ref()
            .and_then(get_first_parameter_binding),
        AnyJsFunction::JsArrowFunctionExpression(arrow) => {
            arrow.parameters().ok().and_then(|params| match params {
                // e.g., `(props) => {}`
                AnyJsArrowFunctionParameters::JsParameters(js_params) => {
                    get_first_parameter_binding(&js_params)
                }
                // e.g., `props => {}`
                AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
                    Some(AnyJsBindingPattern::AnyJsBinding(binding))
                }
            })
        }
        _ => None,
    }
}

/// Determines if a `props` reference occurs in the root scope of the `setup` function.
///
/// In Vue's Composition API, destructuring `props` loses reactivity only in the root
/// scope. Inside nested scopes (like callbacks or returned render functions), it is safe.
///
/// This function walks up the AST from the reference to find the nearest enclosing
/// function and compares it with the `setup` function. If they match, the reference
/// is in the root scope.
fn is_reference_in_root_scope_of_function(
    reference: &JsReferenceIdentifier,
    function: &SetupFunction,
) -> bool {
    let function_syntax = match function {
        SetupFunction::Function(func) => func.syntax(),
        SetupFunction::Method(method) => method.syntax(),
    };

    let nearest_function = reference.syntax().ancestors().find(is_function_like_node);

    nearest_function.is_some_and(|func_node| func_node == *function_syntax)
}

/// Determines if a reference to the `props` parameter is being destructured.
///
/// This function walks up the AST from the reference to check if it appears on the
/// right-hand side of a destructuring pattern in either a variable declaration
/// (`const { a } = props`) or an assignment (`({ a } = props)`).
fn is_reference_in_destructuring(reference: &JsReferenceIdentifier) -> Option<DestructuringInfo> {
    for ancestor in reference.syntax().ancestors() {
        // Case: `const { ... } = props`
        if let Some(declarator) = JsVariableDeclarator::cast_ref(&ancestor)
            && let Ok(id) = declarator.id()
            && let Some(init) = declarator.initializer()
            && let Ok(init_expr) = init.expression()
            && init_expr.range().contains_range(reference.range())
        {
            let initializer = Some(init_expr);
            return match id {
                AnyJsBindingPattern::JsObjectBindingPattern(obj_pattern) => {
                    Some(DestructuringInfo {
                        destructuring_range: obj_pattern.range(),
                        initializer,
                    })
                }
                AnyJsBindingPattern::JsArrayBindingPattern(array_pattern) => {
                    Some(DestructuringInfo {
                        destructuring_range: array_pattern.range(),
                        initializer,
                    })
                }
                _ => None,
            };
        }

        // Case: `({ ... } = props)`
        if let Some(assignment) = JsAssignmentExpression::cast_ref(&ancestor)
            && let Ok(left) = assignment.left()
            && let Ok(right_expr) = assignment.right()
            && right_expr.range().contains_range(reference.range())
        {
            let initializer = Some(right_expr);
            return match left {
                biome_js_syntax::AnyJsAssignmentPattern::JsObjectAssignmentPattern(obj_pattern) => {
                    Some(DestructuringInfo {
                        destructuring_range: obj_pattern.range(),
                        initializer,
                    })
                }
                biome_js_syntax::AnyJsAssignmentPattern::JsArrayAssignmentPattern(
                    array_pattern,
                ) => Some(DestructuringInfo {
                    destructuring_range: array_pattern.range(),
                    initializer,
                }),
                _ => None,
            };
        }
    }

    None
}

/// Checks if destructuring is safe because it uses Vue's reactive APIs.
///
/// Returns `true` if the initializer is a call to `toRefs` or `toRef`.
fn is_safe_reactive_destructuring(
    destructuring_info: &DestructuringInfo,
    model: &SemanticModel,
) -> bool {
    destructuring_info
        .initializer
        .as_ref()
        .is_some_and(|init| is_reactive_api_call(init, model))
}

/// Checks if an expression is a call to `toRefs` imported from a Vue module.
///
/// This function is robust against aliasing (`import { toRefs as myRefs } from 'vue'`)
/// by resolving the binding of the callee and checking its original import name.
fn is_reactive_api_call(expr: &AnyJsExpression, model: &SemanticModel) -> bool {
    // We only care about call expressions, e.g., `toRefs(props)`.
    let Some(call_expr) = expr.as_js_call_expression() else {
        return false;
    };
    let Ok(callee) = call_expr.callee() else {
        return false;
    };

    // Extract the reference identifier from the callee to resolve its binding
    let binding = match &callee {
        AnyJsExpression::JsIdentifierExpression(ident_expr) => {
            ident_expr.name().ok().and_then(|name| model.binding(&name))
        }
        AnyJsExpression::JsStaticMemberExpression(member_expr) => {
            // For cases like `Vue.toRefs(props)`, verify it's a Vue import chain
            if !is_vue_import_chain(&callee, model) {
                return false;
            }
            // Check if the member name is "toRefs"
            return member_expr
                .member()
                .ok()
                .and_then(|member| member.as_js_name().and_then(|name| name.value_token().ok()))
                .is_some_and(|token| token.text_trimmed() == "toRefs");
        }
        AnyJsExpression::JsComputedMemberExpression(computed_member_expr) => {
            // For cases like `Vue["toRefs"](props)`, verify it's a Vue import chain
            let Some(object) = computed_member_expr.object().ok() else {
                return false;
            };
            if !is_vue_import_chain(&object, model) {
                return false;
            }
            // Check if the computed member is the string literal "toRefs"
            return computed_member_expr
                .member()
                .ok()
                .and_then(|member| {
                    member
                        .as_any_js_literal_expression()
                        .and_then(|lit| lit.as_js_string_literal_expression())
                        .and_then(|s| s.inner_string_text().ok())
                })
                .is_some_and(|text| text.text() == "toRefs");
        }
        _ => return false,
    };

    let Some(binding) = binding else {
        return false;
    };

    // Verify the binding is from a Vue module AND that the original imported name was "toRefs".
    is_vue_reactive_function_binding(&binding) && is_imported_as_reactive_function(&binding)
}

/// Checks if a binding is imported from a known Vue module.
fn is_vue_reactive_function_binding(binding: &Binding) -> bool {
    if !binding.is_imported() {
        return false;
    }
    binding
        .syntax()
        .ancestors()
        .find_map(JsImport::cast)
        .and_then(|import_decl| import_decl.source_text().ok())
        .is_some_and(|source| is_vue_module(&source))
}

/// Checks if a binding corresponds to an import of the name "toRefs".
///
/// This function inspects the import specifier to find the original exported name,
fn is_imported_as_reactive_function(binding: &Binding) -> bool {
    binding
        .syntax()
        .parent()
        .and_then(|parent| {
            // Case: `import { toRefs as myRefs } from 'vue'`
            if let Some(specifier) = JsNamedImportSpecifier::cast(parent.clone()) {
                return specifier
                    .name()
                    .ok()
                    .and_then(|name| name.value().ok())
                    .is_some_and(|token| token.text_trimmed() == "toRefs")
                    .then_some(true);
            }
            // Case: `import { toRefs } from 'vue'`
            if let Some(specifier) = JsShorthandNamedImportSpecifier::cast(parent) {
                return specifier
                    .local_name()
                    .ok()
                    .and_then(|name| name.as_js_identifier_binding()?.name_token().ok())
                    .is_some_and(|token| token.text_trimmed() == "toRefs")
                    .then_some(true);
            }
            None
        })
        .unwrap_or(false)
}

/// Recursively checks if an expression chain (e.g., `Vue.toRefs`) originates from a Vue import.
fn is_vue_import_chain(expr: &AnyJsExpression, model: &SemanticModel) -> bool {
    match expr {
        AnyJsExpression::JsIdentifierExpression(ident_expr) => ident_expr
            .name()
            .ok()
            .and_then(|name| model.binding(&name))
            .is_some_and(|binding| is_vue_reactive_function_binding(&binding)),
        AnyJsExpression::JsStaticMemberExpression(member_expr) => member_expr
            .object()
            .ok()
            .is_some_and(|object| is_vue_import_chain(&object, model)),
        _ => false,
    }
}

fn extract_function_from_expression(expr: &AnyJsExpression) -> Option<AnyJsFunction> {
    match expr {
        AnyJsExpression::JsFunctionExpression(func_expr) => {
            Some(AnyJsFunction::JsFunctionExpression(func_expr.clone()))
        }
        AnyJsExpression::JsArrowFunctionExpression(arrow_func) => {
            Some(AnyJsFunction::JsArrowFunctionExpression(arrow_func.clone()))
        }
        _ => None,
    }
}

fn is_member_named_setup(name: &AnyJsObjectMemberName) -> bool {
    name.name().is_some_and(|text| text.text() == "setup")
}

fn get_first_parameter_binding(params: &JsParameters) -> Option<AnyJsBindingPattern> {
    let param = params.items().iter().next()?.ok()?;
    let formal_param = param
        .as_any_js_formal_parameter()?
        .as_js_formal_parameter()?;
    formal_param.binding().ok()
}

fn is_function_like_node(node: &SyntaxNode<JsLanguage>) -> bool {
    matches!(
        node.kind(),
        JsSyntaxKind::JS_FUNCTION_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_METHOD_CLASS_MEMBER
            | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
            | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
    )
}

fn is_vue_module(source: &str) -> bool {
    matches!(source, "vue" | "@vue/reactivity" | "@vue/composition-api")
}
