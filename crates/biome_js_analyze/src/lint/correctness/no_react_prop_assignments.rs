use crate::react::components::{
    AnyPotentialReactComponentDeclaration, ReactComponentInfo, ReactComponentKind,
};
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsExpression, AnyJsStatement, JsParameterList};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_react_prop_assignments::NoReactPropAssignmentsOptions;

declare_lint_rule! {
    /// Disallow assigning to React component props.
    ///
    /// React's `props` are assumed to be immutable, and it is considered bad
    /// practice to assign to properties of the `props` object. When using the
    /// React Compiler, this is even a hard error.
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function Foo(props) {
    /// 	props.bar = "Hello " + props.bar;
    ///
    /// 	return <div>{props.bar}</div>
    /// }
    /// ```
    ///
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// const Foo = function({bar}) {
    ///    bar = "Hello " + bar;
    ///    return <div>{bar}</div>
    ///  }
    /// ```
    ///
    pub NoReactPropAssignments {
        version: "2.0.0",
        name: "noReactPropAssignments",
        language: "jsx",
        sources: &[RuleSource::EslintReactHooks("react-compiler").same()],
        domains: &[RuleDomain::React],
        recommended: false,
    }
}

impl Rule for NoReactPropAssignments {
    type Query = Semantic<AnyPotentialReactComponentDeclaration>;
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = NoReactPropAssignmentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query();
        let model = ctx.model();
        if let Some(component) = ReactComponentInfo::from_declaration(expression.syntax()) {
            match component.kind {
                ReactComponentKind::Function(_) => {
                    let statement = expression
                        .syntax()
                        .ancestors()
                        .find_map(AnyJsStatement::cast)?;

                    match statement {
                        // `function App() {}` or `export function App() {}`
                        AnyJsStatement::JsFunctionDeclaration(function) => {
                            let parameters = function.parameters().ok()?.items();

                            find_param_mutation(&parameters, model)
                        }
                        // `const App = function() {}` or `const App = forwardRef(() => {})` or `const App = memo(function() {})` etc.
                        AnyJsStatement::JsVariableStatement(statement) => {
                            let expression = statement
                                .declaration()
                                .ok()?
                                .declarators()
                                .iter()
                                .find_map(|declarator| {
                                    declarator.ok()?.initializer()?.expression().ok()
                                })?;

                            extract_function_mutations(&expression, model)
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.syntax().text_trimmed_range(),
                markup! {
                    "Mutating component props is not allowed."
                },
            )
            .note(markup! {
                "Consider using a local variable instead."
            }),
        )
    }
}

fn extract_function_mutations(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<AnyJsExpression> {
    match expression {
        AnyJsExpression::JsArrowFunctionExpression(func) => {
            let parameters = func.parameters().ok()?.as_js_parameters()?.items();

            find_param_mutation(&parameters, model)
        }
        AnyJsExpression::JsFunctionExpression(func) => {
            let parameters = func.parameters().ok()?.items();

            find_param_mutation(&parameters, model)
        }
        AnyJsExpression::JsCallExpression(call) => {
            let first = call.arguments().ok()?.args().first()?.ok()?.clone();
            let first_argument = first.syntax().ancestors().find_map(AnyJsExpression::cast)?;

            extract_function_mutations(&first_argument, model)
        }
        _ => None,
    }
}

fn find_param_mutation(
    parameters: &JsParameterList,
    model: &SemanticModel,
) -> Option<AnyJsExpression> {
    let first_param = parameters.first()?.ok()?;
    let formal_param = first_param
        .as_any_js_formal_parameter()?
        .as_js_formal_parameter()?;

    let binding_pattern = formal_param.binding().ok()?;
    let param_binding = binding_pattern.as_any_js_binding()?;
    let param_identifier_binding = param_binding.as_js_identifier_binding()?;
    let param_semantic_binding = model.as_binding(param_identifier_binding);

    param_semantic_binding
        .all_references()
        .take_while(|reference| !reference.is_write())
        .filter_map(|reference| {
            reference
                .syntax()
                .ancestors()
                .find_map(AnyJsStatement::cast)
                .and_then(|stmt| match stmt {
                    AnyJsStatement::JsExpressionStatement(expr_stmt) => Some(expr_stmt),
                    _ => None,
                })
        })
        .find_map(|statement| {
            let left = statement
                .expression()
                .ok()?
                .as_js_assignment_expression()?
                .left()
                .ok()?;
            let object = left
                .as_any_js_assignment()?
                .as_js_static_member_assignment()?
                .as_fields()
                .object
                .ok()?;

            let identifier_expr = object.as_js_identifier_expression()?;
            let reference = identifier_expr.name().ok()?;
            let binding_info = model.binding(&reference)?;

            if binding_info == param_semantic_binding {
                return Some(object);
            }

            None
        })
}
