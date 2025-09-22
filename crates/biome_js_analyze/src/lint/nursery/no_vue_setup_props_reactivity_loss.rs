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
                    "Use 'props.propertyName' or 'toRefs(props)' to maintain reactivity."
                }),
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

            for (name, member) in options_api.iter_declaration_groups() {
                if name.text() == "setup"
                    && let Some(setup_fn) = extract_setup_from_object_member(&member)
                {
                    setup_functions.push(setup_fn);
                }
            }
        }
        AnyVueComponent::DefineComponent(define_component) => {
            if let Some(setup_func_expr) = define_component.setup_func()
                && let Some(func) = extract_function_from_expression(&setup_func_expr)
            {
                setup_functions.push(SetupFunction::Function(func));
            }

            for (name, member) in define_component.iter_declaration_groups() {
                if name.text() == "setup"
                    && let Some(setup_fn) = extract_setup_from_object_member(&member)
                {
                    setup_functions.push(setup_fn);
                }
            }
        }
        AnyVueComponent::CreateApp(create_app) => {
            for (name, member) in create_app.iter_declaration_groups() {
                if name.text() == "setup"
                    && let Some(setup_fn) = extract_setup_from_object_member(&member)
                {
                    setup_functions.push(setup_fn);
                }
            }
        }
        AnyVueComponent::Setup(_) => {}
    }

    setup_functions
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

    let is_setup_member = match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            is_member_named_setup(&method.name().ok()?)
        }
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            is_member_named_setup(&property.name().ok()?)
        }
        _ => return None,
    };

    match (is_setup_member, member) {
        (true, AnyJsObjectMember::JsMethodObjectMember(method)) => {
            Some(SetupFunction::Method(method.clone()))
        }
        (true, AnyJsObjectMember::JsPropertyObjectMember(property)) => {
            let value = property.value().ok()?;
            extract_function_from_expression(&value).map(SetupFunction::Function)
        }
        _ => None,
    }
}

fn is_member_named_setup(name: &biome_js_syntax::AnyJsObjectMemberName) -> bool {
    name.as_js_literal_member_name()
        .and_then(|literal| literal.name().ok())
        .is_some_and(|token| token.text() == "setup")
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
    model
        .as_binding(props_binding)
        .all_reads()
        .filter_map(|reference| reference.syntax().parent())
        .filter_map(biome_js_syntax::JsReferenceIdentifier::cast)
        .filter(|identifier| is_reference_in_root_scope_of_function(identifier, setup_fn))
        .filter_map(|identifier| is_reference_in_destructuring(&identifier))
        .filter(|destructuring_info| !is_safe_reactive_destructuring(destructuring_info, model))
        .map(|destructuring_info| Violation::RootScopeDestructuring {
            destructuring_range: destructuring_info.destructuring_range,
            props_param_range: props_binding.range(),
        })
        .collect()
}

fn is_reference_in_root_scope_of_function(
    reference: &biome_js_syntax::JsReferenceIdentifier,
    function: &SetupFunction,
) -> bool {
    let function_syntax = match function {
        SetupFunction::Function(func) => func.syntax(),
        SetupFunction::Method(method) => method.syntax(),
    };

    reference
        .syntax()
        .ancestors()
        .find(|node| {
            *node == *function_syntax || (is_function_like_node(node) && *node != *function_syntax)
        })
        .is_some_and(|node| node == *function_syntax)
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
    reference.syntax().ancestors().find_map(|node| {
        extract_variable_declarator_info(&node)
            .or_else(|| extract_assignment_expression_info(&node))
    })
}

fn extract_variable_declarator_info(
    node: &biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>,
) -> Option<DestructuringInfo> {
    let declarator = biome_js_syntax::JsVariableDeclarator::cast(node.clone())?;
    let id = declarator.id().ok()?;
    let obj_pattern = id.as_js_object_binding_pattern()?;
    let initializer = declarator
        .initializer()
        .and_then(|init| init.expression().ok());

    Some(DestructuringInfo {
        destructuring_range: obj_pattern.range(),
        initializer,
    })
}

fn extract_assignment_expression_info(
    node: &biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>,
) -> Option<DestructuringInfo> {
    let assignment = biome_js_syntax::JsAssignmentExpression::cast(node.clone())?;
    let left = assignment.left().ok()?;
    let obj_pattern = left.as_js_object_assignment_pattern()?;
    let initializer = assignment.right().ok();

    Some(DestructuringInfo {
        destructuring_range: obj_pattern.range(),
        initializer,
    })
}

fn is_safe_reactive_destructuring(
    destructuring_info: &DestructuringInfo,
    model: &SemanticModel,
) -> bool {
    if let Some(initializer) = &destructuring_info.initializer {
        is_reactive_api_call(initializer, model)
    } else {
        false
    }
}

fn is_reactive_api_call(expr: &AnyJsExpression, model: &SemanticModel) -> bool {
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

    let is_vue_reactive_function = matches!(function_name, "toRefs" | "toRef" | "reactive" | "ref");

    if !is_vue_reactive_function {
        return false;
    }

    let Some(name_node) = ident_expr.name().ok() else {
        return true;
    };

    let Some(binding) = model.binding(&name_node) else {
        return true;
    };

    if !binding.is_imported() {
        return true;
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

    true
}

fn check_setup_function(setup_fn: &SetupFunction, model: &SemanticModel) -> Vec<Violation> {
    let Some(pattern) = get_function_first_parameter(setup_fn) else {
        return Vec::new();
    };

    match pattern {
        AnyJsBindingPattern::JsObjectBindingPattern(obj_pattern) => {
            vec![Violation::ParameterDestructuring(obj_pattern.range())]
        }
        AnyJsBindingPattern::AnyJsBinding(binding) => binding
            .as_js_identifier_binding()
            .map(|props_binding| check_root_scope_destructuring(setup_fn, props_binding, model))
            .unwrap_or_default(),
        _ => Vec::new(),
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
