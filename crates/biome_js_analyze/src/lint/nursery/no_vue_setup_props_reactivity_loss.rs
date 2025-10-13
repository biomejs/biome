use crate::frameworks::vue::vue_component::{AnyPotentialVueComponent, VueComponentQuery};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBindingPattern, AnyJsExpression, AnyJsFunction,
    AnyJsObjectMember, AnyJsObjectMemberName, JsAssignmentExpression, JsCallExpression,
    JsIdentifierBinding, JsMethodObjectMember, JsObjectMemberList, JsParameters,
    JsReferenceIdentifier, JsVariableDeclarator,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

declare_lint_rule! {
    /// Disallow destructuring of `props` passed to `setup` in Vue projects.
    ///
    /// In Vue's Composition API, props must be accessed as `props.propertyName` to maintain
    /// reactivity. Destructuring `props` directly in the `setup` function parameters will
    /// cause the resulting variables to lose their reactive nature.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export default {
    ///   setup({ count }) {
    ///     return () => h('div', count);
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export default {
    ///   setup(props) {
    ///     return () => h('div', props.count);
    ///   }
    /// }
    /// ```
    ///
    pub NoVueSetupPropsReactivityLoss {
        version: "2.2.6",
        name: "noVueSetupPropsReactivityLoss",
        language: "js",
        domains: &[RuleDomain::Vue],
        recommended: false,
        sources: &[RuleSource::EslintVueJs("no-setup-props-reactivity-loss").inspired()],
    }
}

pub enum Violation {
    /// `props` were destructured directly in the function's parameters.
    ParameterDestructuring(TextRange),
    /// `props` were destructured in the root scope of the `setup` function body.
    RootScopeDestructuring {
        destructuring_range: TextRange,
        props_param_range: TextRange,
    },
}

enum SetupFunction {
    Function(AnyJsFunction),
    Method(JsMethodObjectMember),
}

struct DestructuringInfo {
    destructuring_range: TextRange,
}

impl Rule for NoVueSetupPropsReactivityLoss {
    type Query = VueComponentQuery;
    type State = Violation;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        match ctx.query() {
            AnyPotentialVueComponent::JsExportDefaultExpressionClause(export) => {
                let Some(expr) = export.expression().ok() else {
                    return vec![];
                };
                let Some(obj_expr) = expr.as_js_object_expression() else {
                    return vec![];
                };
                check_object_members(&obj_expr.members(), model)
            }
            AnyPotentialVueComponent::JsCallExpression(call_expr) => {
                check_call_expression_setup(call_expr, model)
            }
        }
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

        Some(diagnostic.note(markup! {
            "To preserve reactivity, access props as properties: `props.propertyName`."
        }))
    }
}

fn check_call_expression_setup(
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
                .map(|obj_expr| check_object_members(&obj_expr.members(), model))
        })
        .unwrap_or_default()
}

fn check_object_members(members: &JsObjectMemberList, model: &SemanticModel) -> Vec<Violation> {
    members
        .iter()
        .filter_map(|m| m.ok())
        .find_map(|member| {
            find_setup_function(&member).map(|setup_fn| check_setup_function(&setup_fn, model))
        })
        .unwrap_or_default()
}

fn check_setup_function(setup_fn: &SetupFunction, model: &SemanticModel) -> Vec<Violation> {
    let Some(first_param) = get_first_parameter(setup_fn) else {
        return vec![];
    };

    // Check parameter destructuring first
    match &first_param {
        AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
            return vec![Violation::ParameterDestructuring(pattern.range())];
        }
        AnyJsBindingPattern::JsArrayBindingPattern(pattern) => {
            return vec![Violation::ParameterDestructuring(pattern.range())];
        }
        AnyJsBindingPattern::AnyJsBinding(binding) => {
            // Check for body destructuring
            if let Some(id_binding) = binding.as_js_identifier_binding() {
                return find_body_destructuring_violations(setup_fn, id_binding, model);
            }
        }
    }

    vec![]
}

fn find_body_destructuring_violations(
    setup_fn: &SetupFunction,
    props_binding: &JsIdentifierBinding,
    model: &SemanticModel,
) -> Vec<Violation> {
    let binding = model.as_binding(props_binding);

    binding
        .all_reads()
        .filter_map(|reference| {
            let identifier = JsReferenceIdentifier::cast_ref(reference.syntax())?;

            if !is_in_setup_root_scope(&identifier, setup_fn) {
                return None;
            }

            let destructuring = find_destructuring_info(&identifier)?;

            Some(Violation::RootScopeDestructuring {
                destructuring_range: destructuring.destructuring_range,
                props_param_range: props_binding.range(),
            })
        })
        .collect()
}

fn is_in_setup_root_scope(reference: &JsReferenceIdentifier, setup_fn: &SetupFunction) -> bool {
    let setup_syntax = match setup_fn {
        SetupFunction::Function(func) => func.syntax(),
        SetupFunction::Method(method) => method.syntax(),
    };

    // Find the nearest enclosing function
    let nearest_fn = reference.syntax().ancestors().find(|node| {
        AnyJsFunction::can_cast(node.kind()) || JsMethodObjectMember::can_cast(node.kind())
    });

    // Check if the nearest function is the setup function itself
    nearest_fn.is_some_and(|fn_node| fn_node == *setup_syntax)
}

fn find_destructuring_info(reference: &JsReferenceIdentifier) -> Option<DestructuringInfo> {
    reference.syntax().ancestors().find_map(|ancestor| {
        // Check if it's in a variable declarator
        if let Some(declarator) = JsVariableDeclarator::cast_ref(&ancestor) {
            return check_declarator_destructuring(&declarator, reference);
        }

        // Check if it's in an assignment expression
        if let Some(assignment) = JsAssignmentExpression::cast_ref(&ancestor) {
            return check_assignment_destructuring(&assignment, reference);
        }

        None
    })
}

fn check_declarator_destructuring(
    declarator: &JsVariableDeclarator,
    reference: &JsReferenceIdentifier,
) -> Option<DestructuringInfo> {
    let init = declarator.initializer()?;
    let init_expr = init.expression().ok()?;

    // Ensure reference is in the initializer (right side of =)
    if !init_expr.range().contains_range(reference.range()) {
        return None;
    }

    let id = declarator.id().ok()?;
    let destructuring_range = match id {
        AnyJsBindingPattern::JsObjectBindingPattern(p) => p.range(),
        AnyJsBindingPattern::JsArrayBindingPattern(p) => p.range(),
        _ => return None,
    };

    Some(DestructuringInfo {
        destructuring_range,
    })
}

fn check_assignment_destructuring(
    assignment: &JsAssignmentExpression,
    reference: &JsReferenceIdentifier,
) -> Option<DestructuringInfo> {
    let right_expr = assignment.right().ok()?;

    // Ensure reference is in the right side of =
    if !right_expr.range().contains_range(reference.range()) {
        return None;
    }

    let left = assignment.left().ok()?;
    let destructuring_range = match left {
        biome_js_syntax::AnyJsAssignmentPattern::JsObjectAssignmentPattern(p) => p.range(),
        biome_js_syntax::AnyJsAssignmentPattern::JsArrayAssignmentPattern(p) => p.range(),
        _ => return None,
    };

    Some(DestructuringInfo {
        destructuring_range,
    })
}

fn find_setup_function(member: &AnyJsObjectMember) -> Option<SetupFunction> {
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            if is_named_setup(&method.name().ok()?) {
                Some(SetupFunction::Method(method.clone()))
            } else {
                None
            }
        }
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            if !is_named_setup(&property.name().ok()?) {
                return None;
            }
            let func = get_function_from_expression(&property.value().ok()?)?;
            Some(SetupFunction::Function(func))
        }
        _ => None,
    }
}

fn get_first_parameter(setup_fn: &SetupFunction) -> Option<AnyJsBindingPattern> {
    match setup_fn {
        SetupFunction::Method(method) => {
            let params = method.parameters().ok()?;
            get_first_binding_from_params(&params)
        }
        SetupFunction::Function(func) => get_function_first_parameter(func),
    }
}

fn get_function_first_parameter(func: &AnyJsFunction) -> Option<AnyJsBindingPattern> {
    match func {
        AnyJsFunction::JsArrowFunctionExpression(arrow) => match arrow.parameters().ok()? {
            AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
                Some(AnyJsBindingPattern::AnyJsBinding(binding))
            }
            AnyJsArrowFunctionParameters::JsParameters(params) => {
                get_first_binding_from_params(&params)
            }
        },
        AnyJsFunction::JsFunctionDeclaration(decl) => {
            get_first_binding_from_params(&decl.parameters().ok()?)
        }
        AnyJsFunction::JsFunctionExpression(expr) => {
            get_first_binding_from_params(&expr.parameters().ok()?)
        }
        _ => None,
    }
}

fn get_first_binding_from_params(params: &JsParameters) -> Option<AnyJsBindingPattern> {
    params
        .items()
        .iter()
        .next()?
        .ok()?
        .as_any_js_formal_parameter()?
        .as_js_formal_parameter()?
        .binding()
        .ok()
}

fn get_function_from_expression(expr: &AnyJsExpression) -> Option<AnyJsFunction> {
    match expr {
        AnyJsExpression::JsFunctionExpression(func) => {
            Some(AnyJsFunction::JsFunctionExpression(func.clone()))
        }
        AnyJsExpression::JsArrowFunctionExpression(arrow) => {
            Some(AnyJsFunction::JsArrowFunctionExpression(arrow.clone()))
        }
        _ => None,
    }
}

fn is_named_setup(name: &AnyJsObjectMemberName) -> bool {
    name.name().is_some_and(|text| text.text() == "setup")
}
