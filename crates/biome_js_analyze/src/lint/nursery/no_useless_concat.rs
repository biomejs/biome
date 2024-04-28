use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsBinaryExpression};
use biome_rowan::{AstNode, SyntaxError};

declare_rule! {
    /// Disallow unnecessary concatenation of literals or template literals.
    ///
    /// This rule aims to flag the concatenation of 2 literals when they could be combined into a single literal. Literals can be strings or template literals.
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
    /// const a = "a" + "b" + "c";
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
        let left = node.left();
        let right = node.right();
        let has_string_concatenation = is_literal_string_expression(right);
        let is_left_string_expression = is_literal_string_expression(left);
        let has_useless_concat = has_string_concatenation && is_left_string_expression;

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

fn is_literal_string_expression(node: Result<AnyJsExpression, SyntaxError>) -> bool {
    node.is_ok_and(|node| match node.as_any_js_literal_expression() {
        Some(expression) => expression.as_js_string_literal_expression().is_some(),
        None => false,
    })
}
