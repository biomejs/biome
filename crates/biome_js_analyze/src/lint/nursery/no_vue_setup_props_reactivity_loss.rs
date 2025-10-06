use crate::frameworks::vue::vue_component::{AnyPotentialVueComponent, VueComponentQuery};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBindingPattern, AnyJsExpression, AnyJsFunction,
    AnyJsObjectMember, AnyJsObjectMemberName, JsCallExpression, JsExportDefaultExpressionClause,
    JsMethodObjectMember, JsObjectMemberList, JsParameters,
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
}

/// An abstraction to handle both function expressions and method declarations for `setup`.
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
        analyze_vue_component(ctx.query(), ctx.model())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let Violation::ParameterDestructuring(range) = state;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "Destructuring `props` in the `setup` function parameters loses reactivity."
                },
            )
            .note(markup! {
                "To preserve reactivity, access props as properties: `props.propertyName`."
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
    _model: &SemanticModel,
) -> Vec<Violation> {
    export_clause
        .expression()
        .ok()
        .and_then(|expr| {
            expr.as_js_object_expression()
                .map(|obj_expr| extract_setup_violations_from_members(&obj_expr.members()))
        })
        .unwrap_or_default()
}

fn extract_setup_from_call_expression(
    call_expr: &JsCallExpression,
    _model: &SemanticModel,
) -> Vec<Violation> {
    call_expr
        .arguments()
        .ok()
        .and_then(|args| args.args().iter().next().and_then(|arg| arg.ok()))
        .and_then(|first_arg| {
            first_arg
                .as_any_js_expression()
                .and_then(|e| e.as_js_object_expression())
                .map(|obj_expr| extract_setup_violations_from_members(&obj_expr.members()))
        })
        .unwrap_or_default()
}

fn extract_setup_violations_from_members(members: &JsObjectMemberList) -> Vec<Violation> {
    members
        .iter()
        .filter_map(|member| member.ok())
        .filter_map(|member| extract_setup_from_object_member(&member))
        .flat_map(|setup_fn| check_setup_function(&setup_fn))
        .collect()
}

fn check_setup_function(setup_fn: &SetupFunction) -> Vec<Violation> {
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
        // Case: `setup(props)` - no violation for now
        AnyJsBindingPattern::AnyJsBinding(_) => vec![],
    }
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
