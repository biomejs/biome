use rome_rowan::{AstNode, AstNodeList};

use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsStatement, AnyJsSwitchClause, JsBlockStatement, JsStatementList, JsSwitchStatement,
};

declare_rule! {
    /// Disallow fallthrough of `switch` clauses.
    ///
    /// Switch clauses in `switch` statements fall through by default.
    /// This can lead to unexpected behavior when forgotten.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-fallthrough
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch(bar) {
    /// 	case 0:
    /// 		a();
    /// 	case 1:
    /// 		b()
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// switch(foo) {
    /// 	case 1:
    /// 		doSomething();
    /// 		break;
    /// 	case 2:
    /// 		doSomething();
    /// }
    /// ```
    ///
    pub(crate) NoFallthroughSwitchClause {
        version: "1.0.0",
        name: "noFallthroughSwitchClause",
        recommended: false,
    }
}

impl Rule for NoFallthroughSwitchClause {
    type Query = Ast<JsSwitchStatement>;
    type State = AnyJsSwitchClause;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let mut signals: Self::Signals = Vec::new();
        let mut cases = query.cases().into_iter().peekable();

        while let Some(any_case) = cases.next() {
            let is_last = cases.peek().is_none();

            if is_last {
                continue;
            }

            if case_fell(&any_case) {
                signals.push(any_case);
            }
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.syntax().text_trimmed_range(),
                markup! {
                    "This case is falling through to the next case."
                },
            )
            .note(markup! {
                "Add a `break` or `return` statement to the end of this case to prevent fallthrough."
            }),
        )
    }
}

fn case_fell(case: &AnyJsSwitchClause) -> bool {
    let statements = case.consequent();
    !has_fall_blocker_statement(&statements) && statements.iter().count() != 0
}

fn has_fall_blocker_statement(statements: &JsStatementList) -> bool {
    for statement in statements.iter() {
        if is_fall_blocker_statement(&statement) {
            return true;
        }
        if let Some(block_statement) = JsBlockStatement::cast_ref(statement.syntax()) {
            if has_fall_blocker_statement(&block_statement.statements()) {
                return true;
            }
        }
    }
    false
}

fn is_fall_blocker_statement(statement: &AnyJsStatement) -> bool {
    matches!(
        statement,
        AnyJsStatement::JsBreakStatement(_)
            | AnyJsStatement::JsReturnStatement(_)
            | AnyJsStatement::JsThrowStatement(_)
            | AnyJsStatement::JsContinueStatement(_)
    )
}
