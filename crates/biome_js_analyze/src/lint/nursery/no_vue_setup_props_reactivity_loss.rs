use crate::frameworks::vue::vue_component::{
    AnyPotentialVueComponent, VueComponent, VueComponentQuery,
};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsFunction, JsFileSource, JsIdentifierBinding,
    JsMethodObjectMember,
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
    /// - Destructuring assignment of props in the root scope of setup (unless destructuring from `toRefs(props)`)
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

#[derive(Debug)]
enum SetupFunction {
    Function(AnyJsFunction),
    Method(JsMethodObjectMember),
}

impl Rule for NoVueSetupPropsReactivityLoss {
    type Query = VueComponentQuery;
    type State = Violation;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut violations = Vec::new();

        let component = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type::<JsFileSource>(),
            ctx.file_path(),
        );

        match component {
            Some(component) => {
                for setup_function in find_setup_functions(&component) {
                    violations.extend(check_setup_function(&setup_function, ctx.model()));
                }
            }
            None => {
                violations.extend(check_plain_js_setup_functions(ctx.query(), ctx.model()));
            }
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
                    "Use 'props.propertyName', 'toRefs(props)', or 'toRef(props, \"key\")' to maintain reactivity."
                })
        )
    }
}

fn find_setup_functions(component: &VueComponent) -> Vec<SetupFunction> {
    use crate::frameworks::vue::vue_component::{AnyVueComponent, VueOptionsApiBasedComponent};

    let mut setup_functions = Vec::new();

    match component.kind() {
        AnyVueComponent::OptionsApi(options_api) => {
            if let Some(setup_func_expr) = options_api.setup_func()
                && let Some(func) = extract_function_from_expression(&setup_func_expr)
            {
                setup_functions.push(SetupFunction::Function(func));
            }

            setup_functions.extend(
                options_api
                    .iter_declaration_groups()
                    .filter(|(name, _)| name.text() == "setup")
                    .filter_map(|(_, member)| extract_setup_from_object_member(&member)),
            );
        }
        AnyVueComponent::DefineComponent(define_component) => {
            if let Some(setup_func_expr) = define_component.setup_func()
                && let Some(func) = extract_function_from_expression(&setup_func_expr)
            {
                setup_functions.push(SetupFunction::Function(func));
            }

            setup_functions.extend(
                define_component
                    .iter_declaration_groups()
                    .filter(|(name, _)| name.text() == "setup")
                    .filter_map(|(_, member)| extract_setup_from_object_member(&member)),
            );
        }
        AnyVueComponent::CreateApp(create_app) => {
            setup_functions.extend(
                create_app
                    .iter_declaration_groups()
                    .filter(|(name, _)| name.text() == "setup")
                    .filter_map(|(_, member)| extract_setup_from_object_member(&member)),
            );
        }
        AnyVueComponent::Setup(_) => {}
    }

    {
        use std::collections::HashSet;
        let mut seen: HashSet<TextRange> = HashSet::new();
        let mut deduped = Vec::new();
        for f in setup_functions {
            let range = match &f {
                SetupFunction::Function(func) => func.range(),
                SetupFunction::Method(method) => method.range(),
            };
            if seen.insert(range) {
                deduped.push(f);
            }
        }
        deduped
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

fn extract_setup_from_object_member(
    member: &biome_js_syntax::AnyJsObjectMember,
) -> Option<SetupFunction> {
    use biome_js_syntax::AnyJsObjectMember;

    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            let name = method.name().ok()?;
            is_member_named_setup(&name).then(|| SetupFunction::Method(method.clone()))
        }
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            let name = property.name().ok()?;
            if !is_member_named_setup(&name) {
                return None;
            }
            let value = property.value().ok()?;
            extract_function_from_expression(&value).map(SetupFunction::Function)
        }
        _ => None,
    }
}

fn is_member_named_setup(name: &biome_js_syntax::AnyJsObjectMemberName) -> bool {
    if let Some(lit) = name.as_js_literal_member_name() {
        return lit.value().is_ok_and(|tok| tok.text_trimmed() == "setup");
    }
    false
}
fn get_function_first_parameter(func: &SetupFunction) -> Option<AnyJsBindingPattern> {
    match func {
        SetupFunction::Function(any_func) => get_any_js_function_first_parameter(any_func),
        SetupFunction::Method(method) => {
            let params = method.parameters().ok()?;
            let param = params.items().iter().next()?.ok()?;
            let formal_param = param.as_any_js_formal_parameter()?;
            formal_param.as_js_formal_parameter()?.binding().ok()
        }
    }
}

fn get_any_js_function_first_parameter(func: &AnyJsFunction) -> Option<AnyJsBindingPattern> {
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
    setup_fn: &SetupFunction,
    props_binding: &JsIdentifierBinding,
    model: &SemanticModel,
) -> Vec<Violation> {
    let mut violations = Vec::new();

    for reference in model.as_binding(props_binding).all_reads() {
        if let Some(identifier) =
            biome_js_syntax::JsReferenceIdentifier::cast(reference.syntax().clone())
            && is_reference_in_root_scope_of_function(&identifier, setup_fn)
            && let Some(destructuring_info) = is_reference_in_destructuring(&identifier)
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

fn is_reference_in_root_scope_of_function(
    reference: &biome_js_syntax::JsReferenceIdentifier,
    function: &SetupFunction,
) -> bool {
    let function_syntax = match function {
        SetupFunction::Function(func) => func.syntax(),
        SetupFunction::Method(method) => method.syntax(),
    };

    let nearest_function = reference.syntax().ancestors().find(is_function_like_node);

    nearest_function.is_some_and(|func_node| func_node == *function_syntax)
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
    let current = reference.syntax().clone();

    for ancestor in current.ancestors() {
        if let Some(declarator) = biome_js_syntax::JsVariableDeclarator::cast(ancestor.clone()) {
            let Ok(id) = declarator.id() else {
                continue;
            };
            let has_destructuring = id.as_js_object_binding_pattern().is_some()
                || id.as_js_array_binding_pattern().is_some();

            if !has_destructuring {
                continue;
            }

            let Some(init) = declarator.initializer() else {
                continue;
            };
            let Ok(init_expr) = init.expression() else {
                continue;
            };

            if is_reference_inside_expression(reference, &init_expr) {
                return extract_variable_declarator_info(ancestor);
            }
        }

        if let Some(assignment) = biome_js_syntax::JsAssignmentExpression::cast(ancestor.clone()) {
            let Ok(left) = assignment.left() else {
                continue;
            };
            let has_destructuring = left.as_js_object_assignment_pattern().is_some()
                || left.as_js_array_assignment_pattern().is_some();

            if !has_destructuring {
                continue;
            }

            let Ok(right_expr) = assignment.right() else {
                continue;
            };

            if is_reference_inside_expression(reference, &right_expr) {
                return extract_assignment_expression_info(ancestor);
            }
        }
    }

    None
}

fn is_reference_inside_expression(
    reference: &biome_js_syntax::JsReferenceIdentifier,
    expression: &AnyJsExpression,
) -> bool {
    expression.range().contains_range(reference.range())
}

fn extract_variable_declarator_info(
    node: biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>,
) -> Option<DestructuringInfo> {
    let declarator = biome_js_syntax::JsVariableDeclarator::cast(node)?;
    let id = declarator.id().ok()?;

    if let Some(obj_pattern) = id.as_js_object_binding_pattern() {
        let initializer = declarator
            .initializer()
            .and_then(|init| init.expression().ok());

        return Some(DestructuringInfo {
            destructuring_range: obj_pattern.range(),
            initializer,
        });
    }

    if let Some(array_pattern) = id.as_js_array_binding_pattern() {
        let initializer = declarator
            .initializer()
            .and_then(|init| init.expression().ok());

        return Some(DestructuringInfo {
            destructuring_range: array_pattern.range(),
            initializer,
        });
    }

    None
}

fn extract_assignment_expression_info(
    node: biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>,
) -> Option<DestructuringInfo> {
    let assignment = biome_js_syntax::JsAssignmentExpression::cast(node)?;
    let left = assignment.left().ok()?;

    if let Some(obj_pattern) = left.as_js_object_assignment_pattern() {
        let initializer = assignment.right().ok();
        return Some(DestructuringInfo {
            destructuring_range: obj_pattern.range(),
            initializer,
        });
    }

    if let Some(array_pattern) = left.as_js_array_assignment_pattern() {
        let initializer = assignment.right().ok();
        return Some(DestructuringInfo {
            destructuring_range: array_pattern.range(),
            initializer,
        });
    }

    None
}

fn is_safe_reactive_destructuring(
    destructuring_info: &DestructuringInfo,
    model: &SemanticModel,
) -> bool {
    destructuring_info
        .initializer
        .as_ref()
        .is_some_and(|init| is_reactive_api_call(init, model))
}

fn is_reactive_api_call(expr: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some(call_expr) = expr.as_js_call_expression() else {
        return false;
    };

    let Ok(callee) = call_expr.callee() else {
        return false;
    };

    match &callee {
        AnyJsExpression::JsIdentifierExpression(ident_expr) => {
            let Ok(name) = ident_expr.name() else {
                return false;
            };
            let Ok(token) = name.value_token() else {
                return false;
            };
            let function_name = token.text_trimmed();
            if function_name != "toRefs" && function_name != "toRef" {
                return false;
            }

            if let Some(binding) = model.binding(&name) {
                is_vue_reactive_function_binding(&binding)
            } else {
                false
            }
        }
        AnyJsExpression::JsStaticMemberExpression(member_expr) => {
            let Ok(member) = member_expr.member() else {
                return false;
            };
            let Ok(token) = member.value_token() else {
                return false;
            };
            let property_name = token.text_trimmed();
            if property_name != "toRefs" && property_name != "toRef" {
                return false;
            }

            let Ok(object) = member_expr.object() else {
                return false;
            };

            is_vue_namespace_or_import(&object, model)
        }
        AnyJsExpression::JsComputedMemberExpression(computed_member_expr) => {
            let Ok(member) = computed_member_expr.member() else {
                return false;
            };

            let is_reactive_function = match &member {
                AnyJsExpression::AnyJsLiteralExpression(literal_expr) => {
                    let Some(string_literal) = literal_expr.as_js_string_literal_expression()
                    else {
                        return false;
                    };
                    let Ok(token) = string_literal.value_token() else {
                        return false;
                    };
                    let text = token.text_trimmed();
                    if text.len() >= 2 {
                        let property_name = &text[1..text.len() - 1];
                        property_name == "toRefs" || property_name == "toRef"
                    } else {
                        false
                    }
                }
                _ => false,
            };

            if !is_reactive_function {
                return false;
            }

            let Ok(object) = computed_member_expr.object() else {
                return false;
            };

            is_vue_namespace_or_import(&object, model)
        }
        _ => false,
    }
}

fn is_vue_reactive_function_binding(binding: &biome_js_semantic::Binding) -> bool {
    if !binding.is_imported() {
        return false;
    }

    let binding_node = binding.syntax();
    for ancestor in binding_node.ancestors() {
        let Some(import_decl) = biome_js_syntax::JsImport::cast(ancestor) else {
            continue;
        };
        let Ok(source) = import_decl.source_text() else {
            continue;
        };
        let source_value = source.text();
        return source_value == "vue"
            || source_value == "@vue/reactivity"
            || source_value == "@vue/composition-api";
    }
    false
}

fn is_vue_namespace_or_import(expr: &AnyJsExpression, model: &SemanticModel) -> bool {
    match expr {
        AnyJsExpression::JsIdentifierExpression(ident_expr) => {
            let Ok(name) = ident_expr.name() else {
                return false;
            };

            if let Some(binding) = model.binding(&name) {
                is_vue_reactive_function_binding(&binding)
            } else {
                false
            }
        }
        AnyJsExpression::JsStaticMemberExpression(member_expr) => {
            let Ok(object) = member_expr.object() else {
                return false;
            };
            is_vue_namespace_or_import(&object, model)
        }
        _ => false,
    }
}

fn check_setup_function(setup_fn: &SetupFunction, model: &SemanticModel) -> Vec<Violation> {
    let Some(pattern) = get_function_first_parameter(setup_fn) else {
        return Vec::new();
    };

    match pattern {
        AnyJsBindingPattern::JsObjectBindingPattern(obj_pattern) => {
            vec![Violation::ParameterDestructuring(obj_pattern.range())]
        }
        AnyJsBindingPattern::JsArrayBindingPattern(array_pattern) => {
            vec![Violation::ParameterDestructuring(array_pattern.range())]
        }
        AnyJsBindingPattern::AnyJsBinding(binding) => binding
            .as_js_identifier_binding()
            .map(|props_binding| check_root_scope_destructuring(setup_fn, props_binding, model))
            .unwrap_or_default(),
    }
}

fn check_plain_js_setup_functions(
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
    export_clause: &biome_js_syntax::JsExportDefaultExpressionClause,
    model: &SemanticModel,
) -> Vec<Violation> {
    let Some(expr) = export_clause.expression().ok() else {
        return Vec::new();
    };
    let Some(obj_expr) = expr.as_js_object_expression() else {
        return Vec::new();
    };
    extract_setup_violations_from_members(&obj_expr.members(), model)
}

fn extract_setup_from_call_expression(
    call_expr: &biome_js_syntax::JsCallExpression,
    model: &SemanticModel,
) -> Vec<Violation> {
    let Some(args) = call_expr.arguments().ok() else {
        return Vec::new();
    };
    let Some(first_arg) = args.args().iter().next().and_then(|arg| arg.ok()) else {
        return Vec::new();
    };
    let Some(obj_expr) = first_arg
        .as_any_js_expression()
        .and_then(|e| e.as_js_object_expression())
    else {
        return Vec::new();
    };
    extract_setup_violations_from_members(&obj_expr.members(), model)
}

fn extract_setup_violations_from_members(
    members: &biome_js_syntax::JsObjectMemberList,
    model: &SemanticModel,
) -> Vec<Violation> {
    members
        .iter()
        .filter_map(|member| member.ok())
        .filter_map(|member| extract_setup_from_object_member(&member))
        .flat_map(|setup_fn| check_setup_function(&setup_fn, model))
        .collect()
}
