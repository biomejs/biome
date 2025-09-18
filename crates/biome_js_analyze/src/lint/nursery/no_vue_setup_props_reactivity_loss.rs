use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsFunction, AnyJsObjectMemberName, JsCallExpression,
    JsIdentifierBinding, JsObjectExpression, JsPropertyObjectMember, JsVariableDeclarator,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

declare_lint_rule! {
    /// Disallow destructuring of props passed to setup in Vue projects.
    ///
    /// Vue's Composition API requires that props passed to the `setup` function
    /// maintain reactivity. Destructuring props or using member expressions on props
    /// in the root scope of `setup` will cause the values to lose reactivity.
    ///
    /// This rule reports:
    /// - Direct destructuring of props in setup function parameters
    /// - Destructuring assignment of props in the root scope of setup (unless using `toRefs` or `toRef`)
    ///
    /// Note: destructuring is allowed inside nested functions, callbacks, and
    /// returned render functions where the reactive context is preserved.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// // Destructuring in setup parameters
    /// export default {
    ///   setup({ count }) {
    ///     // count is no longer reactive
    ///     return () => h('div', count)
    ///   }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // Destructuring in setup root scope
    /// export default {
    ///   setup(props) {
    ///     const { count } = props
    ///     // count is no longer reactive
    ///     return () => h('div', count)
    ///   }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // Also works with quoted property names
    /// export default {
    ///   "setup"(props) {
    ///     const { count } = props
    ///     // count is no longer reactive
    ///     return () => h('div', count)
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Keep props reactive
    /// export default {
    ///   setup(props) {
    ///     watch(() => props.count, () => {
    ///       console.log(props.count)
    ///     })
    ///     return () => h('div', props.count)
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// // Using toRefs maintains reactivity
    /// export default {
    ///   setup(props) {
    ///     const { count } = toRefs(props) // OK - count is a ref
    ///     watch(count, () => {
    ///       console.log(count.value)
    ///     })
    ///     return () => h('div', count.value)
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// // Using toRef for individual properties
    /// export default {
    ///   setup(props) {
    ///     const count = toRef(props, 'count') // OK - count is a ref
    ///     return () => h('div', count.value)
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// // Destructuring inside callbacks is OK
    /// export default {
    ///   setup(props) {
    ///     watch(() => props.count, () => {
    ///       const { count } = props // OK inside callback
    ///       console.log(count)
    ///     })
    ///     return () => {
    ///       const { count } = props // OK inside render function
    ///       return h('div', count)
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

#[derive(Debug)]
pub enum Violation {
    ParameterDestructuring(TextRange),
    RootScopeDestructuring {
        destructuring_range: TextRange,
        props_param_range: TextRange,
    },
}

impl Rule for NoVueSetupPropsReactivityLoss {
    type Query = Semantic<AnyJsFunction>;
    type State = Violation;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let function = ctx.query();
        let model = ctx.model();

        if !is_vue_setup_function(function) {
            return vec![];
        }

        let mut violations = Vec::new();

        let first_param_binding = get_function_first_parameter(function);

        let Some(pattern) = first_param_binding else {
            return violations;
        };

        // A props object can be destructured in two ways: directly in the
        // function parameters, or from a variable assignment in the body.
        match pattern {
            AnyJsBindingPattern::JsObjectBindingPattern(obj_pattern) => {
                violations.push(Violation::ParameterDestructuring(obj_pattern.range()));
            }
            AnyJsBindingPattern::AnyJsBinding(binding) => {
                if let Some(props_binding) = binding.as_js_identifier_binding() {
                    violations.extend(check_root_scope_destructuring(
                        function,
                        props_binding,
                        model,
                    ));
                }
            }
            _ => {}
        }

        violations
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            Violation::ParameterDestructuring(range) => RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "Destructuring props in the setup function parameters loses reactivity."
                },
            ),
            Violation::RootScopeDestructuring {
                destructuring_range,
                props_param_range,
            } => RuleDiagnostic::new(
                rule_category!(),
                *destructuring_range,
                markup! {
                    "Destructuring props in the root scope of setup loses reactivity."
                },
            )
            .detail(
                *props_param_range,
                markup! {
                    "The props parameter is defined here."
                },
            ),
        };

        Some(
            diagnostic
                .note(markup! {
                    "In Vue's Composition API, props must be accessed as properties to maintain reactivity."
                })
                .note(markup! {
                    "Use 'props.propertyName' or 'toRefs(props)' to maintain reactivity."
                }),
        )
    }
}

fn is_vue_setup_function(function: &AnyJsFunction) -> bool {
    // Check for method shorthand (setup() {})
    if let Some(method) = function
        .syntax()
        .parent()
        .and_then(biome_js_syntax::JsMethodObjectMember::cast)
    {
        let is_setup = method
            .name()
            .ok()
            .as_ref()
            .and_then(get_object_member_name_text)
            .is_some_and(|name| name == "setup");

        if !is_setup {
            return false;
        }

        return method
            .syntax()
            .ancestors()
            .find_map(JsObjectExpression::cast)
            .is_some_and(|object_expr| is_vue_component_export(&object_expr));
    }

    // Check for property with function value (setup: function() {})
    let Some(property) = function
        .syntax()
        .parent()
        .and_then(JsPropertyObjectMember::cast)
    else {
        return false;
    };

    let is_setup = property
        .name()
        .ok()
        .as_ref()
        .and_then(get_object_member_name_text)
        .is_some_and(|name| name == "setup");

    if !is_setup {
        return false;
    }

    property
        .syntax()
        .ancestors()
        .find_map(JsObjectExpression::cast)
        .is_some_and(|object_expr| is_vue_component_export(&object_expr))
}

fn is_vue_component_export(object_expr: &JsObjectExpression) -> bool {
    if is_direct_default_export(object_expr) {
        return true;
    }

    if is_define_component_export(object_expr) {
        return true;
    }

    if is_named_component_export(object_expr) {
        return true;
    }

    false
}

fn is_direct_default_export(object_expr: &JsObjectExpression) -> bool {
    object_expr
        .syntax()
        .parent()
        .and_then(|parent| {
            parent.parent().filter(|grandparent| {
                grandparent.kind()
                    == biome_js_syntax::JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
            })
        })
        .is_some()
}

fn is_define_component_export(object_expr: &JsObjectExpression) -> bool {
    let Some(call_expr) = object_expr
        .syntax()
        .parent()
        .and_then(JsCallExpression::cast)
    else {
        return false;
    };

    let Ok(callee) = call_expr.callee() else {
        return false;
    };

    let Some(ident_expr) = callee.as_js_identifier_expression() else {
        return false;
    };

    let Ok(name) = ident_expr.name() else {
        return false;
    };

    let Ok(token) = name.value_token() else {
        return false;
    };

    if token.text_trimmed() != "defineComponent" {
        return false;
    }

    call_expr.syntax().ancestors().any(|ancestor| {
        ancestor.kind() == biome_js_syntax::JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
    })
}

fn is_named_component_export(object_expr: &JsObjectExpression) -> bool {
    let Some(declarator) = object_expr
        .syntax()
        .parent()
        .and_then(JsVariableDeclarator::cast)
    else {
        return false;
    };

    declarator
        .syntax()
        .ancestors()
        .any(|ancestor| ancestor.kind() == biome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE)
}

fn get_function_first_parameter(func: &AnyJsFunction) -> Option<AnyJsBindingPattern> {
    match func {
        AnyJsFunction::JsFunctionDeclaration(decl) => {
            let params = decl.parameters().ok()?;
            let param = params.items().iter().next()?.ok()?;
            let formal_param = param.as_any_js_formal_parameter()?;
            formal_param.as_js_formal_parameter()?.binding().ok()
        }
        AnyJsFunction::JsFunctionExpression(expr) => {
            let params = expr.parameters().ok()?;
            let param = params.items().iter().next()?.ok()?;
            let formal_param = param.as_any_js_formal_parameter()?;
            formal_param.as_js_formal_parameter()?.binding().ok()
        }
        AnyJsFunction::JsArrowFunctionExpression(arrow) => {
            let params = arrow.parameters().ok()?;
            match params {
                biome_js_syntax::AnyJsArrowFunctionParameters::JsParameters(js_params) => {
                    let param = js_params.items().iter().next()?.ok()?;
                    let formal_param = param.as_any_js_formal_parameter()?;
                    formal_param.as_js_formal_parameter()?.binding().ok()
                }
                biome_js_syntax::AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
                    Some(AnyJsBindingPattern::AnyJsBinding(binding))
                }
            }
        }
        _ => None,
    }
}

fn check_root_scope_destructuring(
    setup_fn: &AnyJsFunction,
    props_binding: &JsIdentifierBinding,
    model: &SemanticModel,
) -> Vec<Violation> {
    // Find all references to the `props` binding and check if they are
    // part of a destructuring expression within the setup function's root scope.
    let mut violations = Vec::new();

    let props_semantic_binding = model.as_binding(props_binding);

    for reference in props_semantic_binding.all_reads() {
        if let Some(reference_node) = reference.syntax().parent()
            && let Some(identifier) = biome_js_syntax::JsReferenceIdentifier::cast(reference_node)
        {
            if !is_reference_in_root_scope_of_function(&identifier, setup_fn) {
                continue;
            }

            if let Some(destructuring_info) = is_reference_in_destructuring(&identifier) {
                if is_safe_reactive_destructuring(&destructuring_info) {
                    continue;
                }

                violations.push(Violation::RootScopeDestructuring {
                    destructuring_range: destructuring_info.destructuring_range,
                    props_param_range: props_binding.range(),
                });
            }
        }
    }

    violations
}

fn get_object_member_name_text(name: &AnyJsObjectMemberName) -> Option<String> {
    match name {
        AnyJsObjectMemberName::JsLiteralMemberName(literal_name) => {
            let value_token = literal_name.value().ok()?;
            let text = value_token.text_trimmed();

            if (text.starts_with('"') && text.ends_with('"') && text.len() >= 2)
                || (text.starts_with('\'') && text.ends_with('\'') && text.len() >= 2)
            {
                Some(text[1..text.len() - 1].to_string())
            } else {
                Some(value_token.token_text_trimmed().text().to_string())
            }
        }

        AnyJsObjectMemberName::JsComputedMemberName(computed_name) => {
            let expression = computed_name.expression().ok()?;

            if let Some(string_literal) = expression
                .as_any_js_literal_expression()
                .and_then(|literal| literal.as_js_string_literal_expression())
            {
                let value_token = string_literal.value_token().ok()?;
                let text = value_token.text_trimmed();

                if text.len() >= 2
                    && ((text.starts_with('"') && text.ends_with('"'))
                        || (text.starts_with('\'') && text.ends_with('\'')))
                {
                    return Some(text[1..text.len() - 1].to_string());
                }
                Some(value_token.token_text_trimmed().text().to_string())
            } else {
                None
            }
        }

        AnyJsObjectMemberName::JsMetavariable(_) => None,
    }
}

fn is_reference_in_root_scope_of_function(
    reference: &biome_js_syntax::JsReferenceIdentifier,
    function: &AnyJsFunction,
) -> bool {
    let reference_syntax = reference.syntax();
    let function_syntax = function.syntax();

    let mut current = reference_syntax.parent();

    while let Some(node) = current {
        if node == *function_syntax {
            return true;
        }

        if is_function_like_node(&node) && node != *function_syntax {
            return false;
        }

        current = node.parent();
    }

    false
}

fn is_function_like_node(node: &biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>) -> bool {
    use biome_js_syntax::JsSyntaxKind;
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

#[derive(Debug)]
struct DestructuringInfo {
    destructuring_range: TextRange,
    initializer: Option<AnyJsExpression>,
}

fn is_reference_in_destructuring(
    reference: &biome_js_syntax::JsReferenceIdentifier,
) -> Option<DestructuringInfo> {
    let reference_syntax = reference.syntax();

    let mut current = reference_syntax.parent();

    while let Some(node) = current {
        if let Some(declarator) = biome_js_syntax::JsVariableDeclarator::cast(node.clone())
            && let Ok(id) = declarator.id()
            && let Some(obj_pattern) = id.as_js_object_binding_pattern()
        {
            let initializer = declarator
                .initializer()
                .and_then(|init| init.expression().ok());

            return Some(DestructuringInfo {
                destructuring_range: obj_pattern.range(),
                initializer,
            });
        }

        if let Some(assignment) = biome_js_syntax::JsAssignmentExpression::cast(node.clone())
            && let Ok(left) = assignment.left()
            && let Some(obj_pattern) = left.as_js_object_assignment_pattern()
        {
            let initializer = assignment.right().ok();
            return Some(DestructuringInfo {
                destructuring_range: obj_pattern.range(),
                initializer,
            });
        }

        current = node.parent();
    }

    None
}

fn is_safe_reactive_destructuring(destructuring_info: &DestructuringInfo) -> bool {
    if let Some(initializer) = &destructuring_info.initializer {
        is_reactive_api_call(initializer)
    } else {
        false
    }
}

fn is_reactive_api_call(expr: &AnyJsExpression) -> bool {
    let Some(call_expr) = expr.as_js_call_expression() else {
        return false;
    };

    let Ok(callee) = call_expr.callee() else {
        return false;
    };

    let Some(ident_expr) = callee.as_js_identifier_expression() else {
        return false;
    };

    let Ok(name) = ident_expr.name() else {
        return false;
    };

    let Ok(token) = name.value_token() else {
        return false;
    };

    let function_name = token.text_trimmed();

    matches!(
        function_name,
        "toRefs"
            | "toRef"
            | "reactive"
            | "ref"
            | "vueToRefs"
            | "vueToRef"
            | "vueReactive"
            | "vueRef"
    )
}
