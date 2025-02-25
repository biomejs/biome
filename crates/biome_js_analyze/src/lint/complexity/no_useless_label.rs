use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsStatement, JsLabeledStatement, JsSyntaxKind};

use crate::JsRuleAction;
use biome_rowan::{AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Disallow unnecessary labels.
    ///
    /// If a loop contains no nested loops or switches, labeling the loop is unnecessary.
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
    pub NoUselessLabel {
        version: "1.0.0",
        name: "noUselessLabel",
        language: "js",
        sources: &[RuleSource::Eslint("no-extra-label")],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
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
            if is_breakable_statement_kind(parent.kind()) {
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
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
             markup! {"Remove the unnecessary "<Emphasis>"label"</Emphasis>".\nYou can achieve the same result without the label."}.to_owned(),
            mutation,
        ))
    }
}

const fn is_breakable_statement_kind(kind: JsSyntaxKind) -> bool {
    matches!(
        kind,
        JsSyntaxKind::JS_DO_WHILE_STATEMENT
            | JsSyntaxKind::JS_FOR_IN_STATEMENT
            | JsSyntaxKind::JS_FOR_OF_STATEMENT
            | JsSyntaxKind::JS_FOR_STATEMENT
            | JsSyntaxKind::JS_SWITCH_STATEMENT
            | JsSyntaxKind::JS_WHILE_STATEMENT
    )
}
