use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator, JsUnaryOperator,
};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of yoda expressions.
    ///
    /// Yoda expressions can be confusing to some people, the rule forbids the use of it to improve code readability.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if ("red" == value) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (true === value) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (5 != value) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (value === "red") {}
    /// ```
    ///
    /// ```js
    /// if (value === value) {}
    /// ```
    ///
    /// ```js
    /// if (value != 5) {}
    /// ```
    pub NoYodaExpression {
        version: "next",
        name: "noYodaExpression",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoYodaExpression {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let left = node.left().ok();
        let right = node.right().ok();
        let operator = node.operator().ok();
        let has_yoda_expression = is_literal_expression(&left)
            && !is_literal_expression(&right)
            && is_comparison_operator(&operator);

        has_yoda_expression.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Avoid the use of yoda expressions."
                },
            )
            .note(markup! {
                "Yoda expressions can be confusing to some people, invert the expression for better readability."
            }),
        )
    }
}

fn is_literal_expression(expression: &Option<AnyJsExpression>) -> bool {
    match expression {
        Some(AnyJsExpression::AnyJsLiteralExpression(_)) => true,
        Some(AnyJsExpression::JsTemplateExpression(template_expression)) => template_expression
            .elements()
            .into_iter()
            .all(|element| element.as_js_template_chunk_element().is_some()),
        Some(AnyJsExpression::JsUnaryExpression(unary_expression)) => {
            let is_minus_operator =
                matches!(unary_expression.operator(), Ok(JsUnaryOperator::Minus));
            let is_number_expression = matches!(
                unary_expression.argument(),
                Ok(AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                ))
            );

            is_minus_operator && is_number_expression
        }
        _ => false,
    }
}

fn is_comparison_operator(operator: &Option<JsBinaryOperator>) -> bool {
    match operator {
        Some(JsBinaryOperator::LessThan) => true,
        Some(JsBinaryOperator::GreaterThan) => true,
        Some(JsBinaryOperator::LessThanOrEqual) => true,
        Some(JsBinaryOperator::GreaterThanOrEqual) => true,
        Some(JsBinaryOperator::Equality) => true,
        Some(JsBinaryOperator::StrictEquality) => true,
        Some(JsBinaryOperator::Inequality) => true,
        Some(JsBinaryOperator::StrictInequality) => true,
        _ => false,
    }
}
