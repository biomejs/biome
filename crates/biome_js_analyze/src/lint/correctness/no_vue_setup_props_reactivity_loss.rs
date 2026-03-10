use crate::frameworks::vue::vue_component::{AnyPotentialVueComponent, VueComponentQuery};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBindingPattern, AnyJsExpression, AnyJsFunction,
    AnyJsObjectMember, AnyJsObjectMemberName, JsCallExpression, JsMethodObjectMember,
    JsObjectMemberList, JsParameters,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_vue_setup_props_reactivity_loss::NoVueSetupPropsReactivityLossOptions;

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
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        recommended: false,
        sources: &[RuleSource::EslintVueJs("no-setup-props-reactivity-loss").inspired()],
    }
}

pub struct Violation(TextRange);

impl Violation {
    fn range(&self) -> TextRange {
        self.0
    }
}

enum SetupFunction {
    Function(AnyJsFunction),
    Method(JsMethodObjectMember),
}

impl Rule for NoVueSetupPropsReactivityLoss {
    type Query = VueComponentQuery;
    type State = Violation;
    type Signals = Vec<Self::State>;
    type Options = NoVueSetupPropsReactivityLossOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            // Case: export default { setup(props) { ... } }
            AnyPotentialVueComponent::JsExportDefaultExpressionClause(export) => {
                let Some(expr) = export.expression().ok() else {
                    return vec![];
                };
                let Some(obj_expr) = expr.as_js_object_expression() else {
                    return vec![];
                };
                check_object_members(&obj_expr.members())
            }
            // Case: export default defineComponent({ setup(props) { ... } })
            AnyPotentialVueComponent::JsCallExpression(call_expr) => {
                check_call_expression_setup(call_expr)
            }
            _ => Self::Signals::default(),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
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

fn check_call_expression_setup(call_expr: &JsCallExpression) -> Vec<Violation> {
    if let Ok(args) = call_expr.arguments()
        && let Some(Ok(arg)) = args.args().iter().next()
        && let Some(expr) = arg.as_any_js_expression()
        && let Some(obj_expr) = expr.as_js_object_expression()
    {
        check_object_members(&obj_expr.members())
    } else {
        vec![]
    }
}

fn check_object_members(members: &JsObjectMemberList) -> Vec<Violation> {
    members
        .iter()
        .filter_map(|m| m.ok())
        .filter_map(|member| find_setup_function(&member))
        .filter_map(|setup| check_setup_params(&setup))
        .collect()
}

fn check_setup_params(setup_fn: &SetupFunction) -> Option<Violation> {
    let first_param = get_first_parameter(setup_fn)?;

    match first_param {
        AnyJsBindingPattern::JsObjectBindingPattern(obj) => Some(Violation(obj.range())),
        AnyJsBindingPattern::JsArrayBindingPattern(arr) => Some(Violation(arr.range())),
        AnyJsBindingPattern::AnyJsBinding(_) => None,
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

fn find_setup_function(member: &AnyJsObjectMember) -> Option<SetupFunction> {
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => method
            .name()
            .ok()
            .filter(is_named_setup)
            .map(|_| SetupFunction::Method(method.clone())),
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            let name = property.name().ok()?;
            if !is_named_setup(&name) {
                return None;
            }
            let value = property.value().ok()?;
            let func = get_function_from_expression(&value)?;
            Some(SetupFunction::Function(func))
        }
        _ => None,
    }
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
