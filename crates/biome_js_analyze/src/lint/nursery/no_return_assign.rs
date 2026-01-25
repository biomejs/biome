use biome_analyze::{
    QueryMatch, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsArrowFunctionExpression, JsAssignmentExpression, JsClassExpression,
    JsFunctionExpression, JsReturnStatement,
};
use biome_rowan::{AstNode, TextRange, WalkEvent, declare_node_union};
use biome_rule_options::no_return_assign::NoReturnAssignOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow assignments in return statements.
    ///
    /// In return statements, it is common to mistype a comparison operator (such as `==`) as an assignment operator (such as `=`).
    /// Moreover, the use of assignments in a return statement is confusing.
    /// Return statements are often considered side-effect free.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f(a) {
    ///     return a = 1;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function f(a) {
    ///     a = 1;
    ///     return a;
    /// }
    /// ```
    ///
    /// ```js
    /// function f(a) {
    ///     return a == 1;
    /// }
    /// ```
    ///
    pub NoReturnAssign {
        version: "2.3.11",
        name: "noReturnAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-return-assign").same()],
        recommended: false,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub AnyReturn = JsReturnStatement | JsArrowFunctionExpression
}

impl Rule for NoReturnAssign {
    type Query = Semantic<AnyReturn>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = NoReturnAssignOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        run_options(ctx).unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup!{
                    <Emphasis>"Return statements"</Emphasis>" should not contain "<Emphasis>"assignments"</Emphasis>"."
                },
            ).note(markup! {
                "Assignments inside return statements are easy to mistake for comparison operators (`==`), "
                "and add unexpected side effects to normally-pure code."
                "\nIf the assignment is intentional, move it outside of the return statement."
            }))
    }
}

fn run_options(ctx: &RuleContext<NoReturnAssign>) -> Option<Vec<TextRange>> {
    match ctx.query() {
        AnyReturn::JsReturnStatement(query) => Some(traverse_expression(&query.argument()?)),

        AnyReturn::JsArrowFunctionExpression(query) => Some(traverse_expression(
            query.body().ok()?.as_any_js_expression()?,
        )),
    }
}

fn traverse_expression(root: &AnyJsExpression) -> Vec<TextRange> {
    let mut signal = Vec::new();
    let mut iter = root.syntax().preorder();

    while let Some(event) = iter.next() {
        if let WalkEvent::Enter(node) = event {
            if JsAssignmentExpression::can_cast(node.kind()) {
                signal.push(node.text_range());
            }

            // Skip function and class boundaries - assignments inside nested
            // functions/classes are not part of this return statement
            if JsArrowFunctionExpression::can_cast(node.kind())
                || JsFunctionExpression::can_cast(node.kind())
                || JsClassExpression::can_cast(node.kind())
            {
                iter.skip_subtree();
                continue;
            }

            let is_expression = AnyJsExpression::can_cast(node.kind());

            if !is_expression {
                iter.skip_subtree();
            }
        }
    }
    signal
}
