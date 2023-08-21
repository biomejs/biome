use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsCaseClause, JsDefaultClause};
use rome_rowan::{AstNode, AstNodeList, BatchMutationExt, Direction, SyntaxElement};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow useless `case` in `switch` statements.
    ///
    /// A `switch` statement can optionally have a `default` clause.
    ///
    /// The `default` clause will be still executed only if there is no match in the `case` clauses.
    /// An empty `case` clause that precedes the `default` clause is thus useless.
    ///
    /// Source: https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/no-useless-switch-case.md
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     case 0:
    ///     default:
    ///         break;
    ///     case 1:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     default:
    ///     case 0:
    ///         break;
    ///     case 1:
    ///         break;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (foo) {
    ///     case 0:
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js
    /// switch (foo) {
    ///     case 0:
    ///         break;
    /// }
    /// ```
    ///
    pub(crate) NoUselessSwitchCase {
        version: "1.0.0",
        name: "noUselessSwitchCase",
        recommended: true,
    }
}

impl Rule for NoUselessSwitchCase {
    type Query = Ast<JsDefaultClause>;
    type State = JsCaseClause;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let default_clause = ctx.query();
        let it = default_clause
            .syntax()
            .siblings(Direction::Prev)
            .filter_map(JsCaseClause::cast)
            .take_while(|case| case.consequent().is_empty());
        if default_clause.consequent().is_empty() {
            // The default clause is directly followed by at least a case. e.g.
            //
            // ```js
            // switch (foo) {
            //   default:
            //   case 1:
            //   case 2:
            //     break;
            // }
            // ```
            //
            it.chain(
                default_clause
                    .syntax()
                    .siblings(Direction::Next)
                    .filter_map(JsCaseClause::cast)
                    .take_while(|case| case.consequent().is_empty()),
            )
            .chain(
                default_clause
                    .syntax()
                    .siblings(Direction::Next)
                    .filter_map(JsCaseClause::cast)
                    .find(|case| !case.consequent().is_empty()),
            )
            .collect()
        } else {
            it.collect()
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, useless_case: &Self::State) -> Option<RuleDiagnostic> {
        let default_clause = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                useless_case.range(),
                markup! {
                    "Useless "<Emphasis>"case clause"</Emphasis>"."
                },
            )
            .detail(
                default_clause.range(),
                markup! {
                    "because the "<Emphasis>"default clause"</Emphasis>" is present:"
                },
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, useless_case: &Self::State) -> Option<JsRuleAction> {
        let default_clause = ctx.query();
        let mut mutation = ctx.root().begin();
        let consequent = useless_case.consequent();
        if consequent.len() > 0 {
            let default_clause_colon_token = default_clause.colon_token().ok()?;
            let new_default_clause = default_clause
                .to_owned()
                .with_consequent(consequent)
                .with_colon_token(default_clause_colon_token.append_trivia_pieces(
                    useless_case.colon_token().ok()?.trailing_trivia().pieces(),
                ));
            mutation.remove_node(default_clause.to_owned());
            mutation.replace_element(
                SyntaxElement::Node(useless_case.syntax().to_owned()),
                SyntaxElement::Node(new_default_clause.syntax().to_owned()),
            );
        } else {
            mutation.remove_node(useless_case.to_owned());
        }
        Some(JsRuleAction {
            mutation,
            message: markup! {"Remove the useless "<Emphasis>"case"</Emphasis>"."}.to_owned(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
        })
    }
}
