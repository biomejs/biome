use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsBinaryExpression, JsBinaryOperator};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow unnecessary concatenation of literals or template literals.
    ///
    /// This rule aims to flag the concatenation of 2 literals when they could be combined into a single literal. Literals can be strings or template literals.
    /// Concatenation of multiple strings in different lines to prevent big line widths are allowed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = "a" + "b";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = "a" + "b" + "c";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = foo + "a" + "b";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = foo + "a" + ("b" + "c");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = 1 + 1;
    /// ```
    ///
    /// ```js
    /// const a = 1 * '2';
    /// ```
    ///
    /// ```js
    /// const a = 1 - 2;
    /// ```
    ///
    /// ```js
    /// const a = foo + bar;
    /// ```
    ///
    /// ```js
    /// const a = 'foo' + bar;
    /// ```
    ///
    /// ```js
    /// const a = 'foo' +
    ///           'bar'
    /// ```
    pub NoUselessConcat {
        version: "next",
        name: "noUselessConcat",
        recommended: false,
    }
}

impl Rule for NoUselessConcat {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let has_left_string_expression = is_string_expression(node.left().ok())
            || is_binary_expression_with_literal_string(node.left().ok())
            || is_parenthesized_concatenation(node.left().ok());
        let has_useless_concat = has_left_string_expression
            && is_concatenation(node)
            && !is_stylistic_concatenation(node);

        has_useless_concat.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Useless string concatenation."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}

fn is_string_expression(expression: Option<AnyJsExpression>) -> bool {
    expression.is_some_and(|node| {
        match (
            node.as_any_js_literal_expression(),
            node.as_js_template_expression(),
        ) {
            (Some(literal_expression), _) => literal_expression
                .as_js_string_literal_expression()
                .is_some(),
            (_, Some(_template_expression)) => true,
            _ => false,
        }
    })
}

fn is_binary_expression_with_literal_string(expression: Option<AnyJsExpression>) -> bool {
    if let Some(AnyJsExpression::JsBinaryExpression(binary_expression)) = expression {
        return is_string_expression(binary_expression.right().ok());
    }

    false
}

fn is_concatenation(binary_expression: &JsBinaryExpression) -> bool {
    let operator = binary_expression.operator().ok();
    let is_plus_operator = matches!(operator, Some(JsBinaryOperator::Plus));
    let is_string_expression = is_string_expression(binary_expression.right().ok())
        || is_parenthesized_concatenation(binary_expression.right().ok());

    is_plus_operator && is_string_expression
}

/// Returns if the passed `JsBinaryExpression` has a multiline string concatenation
fn is_stylistic_concatenation(binary_expression: &JsBinaryExpression) -> bool {
    let operator = binary_expression.operator().ok();
    let is_plus_operator = matches!(operator, Some(JsBinaryOperator::Plus));
    let has_newline_in_right = binary_expression.right().is_ok_and(|right| {
        match (
            right.as_any_js_literal_expression(),
            right.as_js_template_expression(),
        ) {
            (Some(literal_expression), _) => literal_expression
                .as_js_string_literal_expression()
                .is_some_and(|string_literal_expression| {
                    string_literal_expression
                        .as_fields()
                        .value_token
                        .is_ok_and(|token| token.has_leading_newline())
                }),
            (_, Some(template_expression)) => template_expression
                .l_tick_token()
                .is_ok_and(|token| token.has_leading_newline()),
            _ => false,
        }
    });

    is_plus_operator && has_newline_in_right
}

fn is_parenthesized_concatenation(expression: Option<AnyJsExpression>) -> bool {
    if let Some(AnyJsExpression::JsParenthesizedExpression(parenthesized_expression)) = expression {
        return is_binary_expression_with_literal_string(
            parenthesized_expression.expression().ok(),
        );
    }

    false
}
