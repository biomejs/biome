use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsSwitchClause, JsCaseClause, JsDefaultClause};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, Direction};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow useless `case` in `switch` statements.
    ///
    /// A `switch` statement can optionally have a `default` clause.
    ///
    /// The `default` clause will be still executed only if there is no match in the `case` clauses.
    /// An empty `case` clause that precedes the `default` clause is thus useless.
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
    pub NoUselessSwitchCase {
        version: "1.0.0",
        name: "noUselessSwitchCase",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-useless-switch-case")],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessSwitchCase {
    type Query = Ast<JsDefaultClause>;
    type State = JsCaseClause;
    type Signals = Box<[Self::State]>;
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
            .collect::<Vec<_>>()
        } else {
            it.collect::<Vec<_>>()
        }
        .into_boxed_slice()
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
        let useless_case = useless_case.clone();
        if consequent.len() > 0 {
            let default_clause_colon_token = default_clause.colon_token().ok()?;
            let new_default_clause = default_clause
                .clone()
                .with_consequent(consequent)
                .with_colon_token(default_clause_colon_token.append_trivia_pieces(
                    useless_case.colon_token().ok()?.trailing_trivia().pieces(),
                ));
            mutation.remove_node(default_clause.clone());
            mutation.replace_node(
                AnyJsSwitchClause::from(useless_case),
                new_default_clause.into(),
            );
        } else {
            mutation.remove_node(useless_case);
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {"Remove the useless "<Emphasis>"case"</Emphasis>"."}.to_owned(),
            mutation,
        ))
    }
}
