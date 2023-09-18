use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsStatement, JsElseClause, JsIfStatement, JsStatementList};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow `else` block when the `if` block breaks early.
    ///
    /// If an `if` block breaks early using a breaking statement (`return`, `break`, `continue`, or `throw`),
    /// then the `else` block becomes useless.
    /// Its contents can be placed outside of the block.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// while (x > 0) {
    ///     if (f(x)) {
    ///         break;
    ///     } else {
    ///         x++
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///     if (x < 0) {
    ///         return 0;
    ///     } else {
    ///         return x;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///     if (x < 0) {
    ///         throw new RangeError();
    ///     } else {
    ///         return x;
    ///     }
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// function f() {
    ///     if (x < 0) {
    ///         return 0;
    ///     }
    ///     return x;
    /// }
    /// ```
    pub(crate) NoUselessElse {
        version: "next",
        name: "noUselessElse",
        recommended: true,
    }
}

impl Rule for NoUselessElse {
    type Query = Ast<JsElseClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let else_clause = ctx.query();
        let if_stmt = else_clause.parent::<JsIfStatement>()?;
        let mut stmt_stack = vec![(if_stmt.consequent().ok()?, ScopeMetadata::default())];
        while let Some((stmt, metadata)) = stmt_stack.pop() {
            match stmt {
                AnyJsStatement::JsBlockStatement(block_stmt) => {
                    let Some(last) = block_stmt.statements().iter().last() else {
                        // empty statement block
                        return None;
                    };
                    stmt_stack.push((last, metadata));
                }
                AnyJsStatement::JsBreakStatement(_) => {
                    if metadata.is_breakable {
                        // We are inside a breakable structure (switch statement)
                        // that we saw in a previous iteration.
                        return None;
                    }
                }
                AnyJsStatement::JsContinueStatement(_)
                | AnyJsStatement::JsReturnStatement(_)
                | AnyJsStatement::JsThrowStatement(_) => {}
                AnyJsStatement::JsIfStatement(if_stmt) => {
                    let Some(else_clause) = if_stmt.else_clause() else {
                        // No else clause
                        return None;
                    };
                    stmt_stack.push((if_stmt.consequent().ok()?, metadata));
                    stmt_stack.push((else_clause.alternate().ok()?, metadata));
                }
                AnyJsStatement::JsSwitchStatement(switch_stmt) => {
                    // To simplify, We do not take fallthoughs into account.
                    // Thus, this can miss some useless else.
                    let cases = switch_stmt.cases();
                    let Some(last_case) = cases.last() else {
                        // Empty switch
                        return None;
                    };
                    if last_case.consequent().is_empty() {
                        return None;
                    }
                    for switch_clause in cases.iter() {
                        if let Some(last) = switch_clause.consequent().last() {
                            stmt_stack.push((last, ScopeMetadata { is_breakable: true }));
                        }
                    }
                }
                _ => {
                    // labeled statements, loops, try-catch, with statement, and others
                    return None;
                }
            }
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let else_clause = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                else_clause.range(),
                markup! {
                    "This "<Emphasis>"else"</Emphasis>" clause can be omitted."
                },
            )
            .detail(
                else_clause.syntax().parent()?.text_trimmed_range(),
                markup! {
                    "This "<Emphasis>"if"</Emphasis>" statement uses an early breaking statement."
                },
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let else_clause = ctx.query();
        let if_stmt = else_clause.parent::<JsIfStatement>()?;
        if let Some(stmts_list) = JsStatementList::cast(if_stmt.syntax().parent()?) {
            let else_alternative = else_clause.alternate().ok()?;
            let if_pos = stmts_list
                .iter()
                .position(|x| x.syntax() == if_stmt.syntax())?;
            let new_if_stmt = AnyJsStatement::JsIfStatement(if_stmt.clone().with_else_clause(None))
                .with_trailing_trivia_pieces([])?;
            let prev_stmts = stmts_list.iter().take(if_pos).chain([new_if_stmt]);
            let next_stmts = stmts_list.iter().skip(if_pos + 1);
            // We collect the statements because `chain` is not able to produce an `ExactSizeIterator`.
            let new_stmts: Vec<_> =
                if let AnyJsStatement::JsBlockStatement(else_block_stmts) = else_alternative {
                    prev_stmts
                        .chain(else_block_stmts.statements().iter())
                        .chain(next_stmts)
                        .collect()
                } else {
                    prev_stmts
                        .chain([else_alternative])
                        .chain(next_stmts)
                        .collect()
                };
            let new_stmts_list = make::js_statement_list(new_stmts);
            let mut mutation = ctx.root().begin();
            mutation.replace_node_discard_trivia(stmts_list, new_stmts_list);
            return Some(JsRuleAction {
                category: ActionCategory::QuickFix,
                applicability: Applicability::MaybeIncorrect,
                message: markup! { "Omit the "<Emphasis>"else"</Emphasis>" clause." }.to_owned(),
                mutation,
            });
        }
        None
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct ScopeMetadata {
    // We are inside a breakable structure
    is_breakable: bool,
}
