use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Applicability;
use rome_js_syntax::{
    AnyJsStatement, JsDoWhileStatement, JsForInStatement, JsForOfStatement, JsForStatement,
    JsLabeledStatement, JsSwitchStatement, JsWhileStatement,
};

use crate::JsRuleAction;
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt};

declare_rule! {
    /// Disallow unnecessary labels.
    ///
    /// If a loop contains no nested loops or switches, labeling the loop is unnecessary.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-extra-label
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// loop: while(a) {
    ///     break loop;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// outer: while(a) {
    ///     while(b) {
    ///         break outer;
    ///     }
    /// }
    /// ```
    ///
    pub(crate) NoUselessLabel {
        version: "1.0.0",
        name: "noUselessLabel",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) JsBreakableStatement =
        JsDoWhileStatement |
        JsForInStatement |
        JsForOfStatement |
        JsForStatement |
        JsSwitchStatement |
        JsWhileStatement
}

impl Rule for NoUselessLabel {
    type Query = Ast<AnyJsStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let stmt = ctx.query();
        let label_token = match stmt {
            AnyJsStatement::JsBreakStatement(x) => x.label_token(),
            AnyJsStatement::JsContinueStatement(x) => x.label_token(),
            _ => None,
        }?;
        let label = label_token.text_trimmed();
        for parent in stmt.syntax().ancestors() {
            if JsBreakableStatement::can_cast(parent.kind()) {
                if let Some(labeled_stmt) = JsLabeledStatement::cast(parent.parent()?) {
                    if labeled_stmt.label_token().ok()?.text_trimmed() == label {
                        return Some(());
                    }
                }
                break;
            } else if let Some(labeled_stmt) = JsLabeledStatement::cast(parent) {
                if labeled_stmt.label_token().ok()?.text_trimmed() == label {
                    break;
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let stmt = ctx.query();
        let label_token = match stmt {
            AnyJsStatement::JsBreakStatement(x) => x.label_token(),
            AnyJsStatement::JsContinueStatement(x) => x.label_token(),
            _ => None,
        }?;
        Some(RuleDiagnostic::new(
            rule_category!(),
            label_token.text_trimmed_range(),
            markup! {
                "Unnecessary "<Emphasis>"label"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let stmt = ctx.query();
        let (stmt_token, label_token) = match stmt {
            AnyJsStatement::JsBreakStatement(x) => (x.break_token().ok()?, x.label_token()?),
            AnyJsStatement::JsContinueStatement(x) => (x.continue_token().ok()?, x.label_token()?),
            _ => return None,
        };
        // We want to remove trailing spaces and keep all comments that follows `stmt_token`
        // e.g. `break /* a comment */  ` to `break /* a comment */`.
        // This requires to traverse the trailing trivia in reverse order.
        // We keep trailing trivia of `label_stmt`
        // e.g. `break label // a comment` -> `break // a comment`
        // We do not keep leading trivia of `label_stmt` because we assume that they are associated to the label.
        let new_stmt_token = stmt_token
            .trim_trailing_trivia()
            .append_trivia_pieces(label_token.trailing_trivia().pieces());
        let mut mutation = ctx.root().begin();
        mutation.remove_token(label_token);
        mutation.replace_token_discard_trivia(stmt_token, new_stmt_token);
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! {"Remove the unnecessary "<Emphasis>"label"</Emphasis>".\nYou can achieve the same result without the label."}.to_owned(),
            mutation,
        })
    }
}
