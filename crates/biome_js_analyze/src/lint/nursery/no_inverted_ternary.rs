use crate::JsRuleAction;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{JsBinaryOperator, JsConditionalExpression, T};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_inverted_ternary::NoInvertedTernaryOptions;

declare_lint_rule! {
    /// Disallow inverted ternary conditions.
    ///
    /// An inverted ternary uses an inequality check (`!=` or `!==`) and puts
    /// the "positive" branch in the consequent, which hurts readability.
    /// Prefer a direct equality check and swap the branches.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const value = i !== data.depth - 1 ? 18 : 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = foo != bar ? first : second;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const value = i === data.depth - 1 ? 0 : 18;
    /// ```
    ///
    /// ```js
    /// const result = foo === bar ? second : first;
    /// ```
    ///
    pub NoInvertedTernary {
        version: "next",
        name: "noInvertedTernary",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoInvertedTernary {
    type Query = Ast<JsConditionalExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoInvertedTernaryOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let test_expression = node.test().ok()?;
        let test = test_expression.as_js_binary_expression()?;
        matches!(
            test.operator().ok()?,
            JsBinaryOperator::Inequality | JsBinaryOperator::StrictInequality
        )
        .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Avoid inverted ternary conditions."
                },
            )
            .note(markup! {
                "Use an equality check in the test and swap the two branches for better readability."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let test_expression = node.test().ok()?;
        let test = test_expression.as_js_binary_expression()?;
        let flipped_operator = match test.operator().ok()? {
            JsBinaryOperator::StrictInequality => T![===],
            JsBinaryOperator::Inequality => T![==],
            _ => return None,
        };

        let left = test.left().ok()?;
        let right = test.right().ok()?;
        let operator_token = test.operator_token().ok()?;
        let consequent = node.consequent().ok()?;
        let alternate = node.alternate().ok()?;

        let new_operator = make::token(flipped_operator)
            .prepend_trivia_pieces(operator_token.leading_trivia().pieces())
            .append_trivia_pieces(operator_token.trailing_trivia().pieces());

        let new_test = make::js_binary_expression(left, new_operator, right).into();
        let new_ternary = make::js_conditional_expression(
            new_test,
            node.question_mark_token().ok()?,
            alternate,
            node.colon_token().ok()?,
            consequent,
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(node.clone(), new_ternary);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! { "Use a direct ternary condition." }.to_owned(),
            mutation,
        ))
    }
}
