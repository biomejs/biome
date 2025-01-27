use std::borrow::Cow;

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsStatement, JsIfStatement, JsStatementList, JsSyntaxKind};
use biome_rowan::{
    chain_trivia_pieces, trim_leading_trivia_pieces, AstNode, AstNodeList, BatchMutationExt,
    SyntaxNodeOptionExt,
};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow `else` block when the `if` block breaks early.
    ///
    /// If an `if` block breaks early using a breaking statement (`return`, `break`, `continue`, or `throw`),
    /// then the `else` block becomes useless.
    /// Its contents can be placed outside of the block.
    ///
    /// If an `if` block breaks early using a breaking statement (`return`, `break`, `continue`, or `throw`),
    /// then the `else` block becomes unnecessary.
    /// This is because the content of the `else` block will never be executed in conjunction with the `if` block,
    /// as the breaking statement ensures the control flow exits the `if` block immediately.
    /// Therefore, the `else` block is redundant, and its content can be placed outside of the block,
    /// reducing the indentation level by one.
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
    /// function f(x) {
    ///     if (x < 0) {
    ///         return 0;
    ///     } else {
    ///         return x;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(x) {
    ///     if (x < 0) {
    ///         throw new RangeError();
    ///     } else {
    ///         return x;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function f(x) {
    ///     if (x < 0) {
    ///         return 0;
    ///     }
    ///     return x;
    /// }
    /// ```
    ///
    /// ```js
    /// function f(x) {
    ///     if (x < 0) {
    ///         console.info("negative number");
    ///     } else if (x > 0) {
    ///         return x;
    ///     } else {
    ///         console.info("number 0");
    ///     }
    /// }
    /// ```
    pub NoUselessElse {
        version: "1.3.0",
        name: "noUselessElse",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-else-return"),
            RuleSource::Clippy("redundant_else 	"),
        ],
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessElse {
    type Query = Ast<JsIfStatement>;
    type State = JsIfStatement;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut result = Vec::new();
        let if_stmt = ctx.query();
        // Check an `if` statement only once.
        if if_stmt.syntax().parent().kind() == Some(JsSyntaxKind::JS_ELSE_CLAUSE) {
            return result.into_boxed_slice();
        }
        let mut if_stmt = Cow::Borrowed(if_stmt);
        while let (Ok(if_consequent), Some(else_clause)) =
            (if_stmt.consequent(), if_stmt.else_clause())
        {
            // if the `if` statement doesn't break early, stop there
            if breaks_early(if_consequent).is_none() {
                break;
            }
            // Otherwise, report the `else` clause as useless
            result.push(if_stmt.into_owned());
            // And check the following `if` statement (if any)
            let Some(stmt) = else_clause
                .alternate()
                .ok()
                .and_then(|alternate| JsIfStatement::cast(alternate.into_syntax()))
            else {
                break;
            };
            if_stmt = Cow::Owned(stmt);
        }
        result.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, if_stmt: &Self::State) -> Option<RuleDiagnostic> {
        let else_clause = if_stmt.else_clause()?;
        Some(RuleDiagnostic::new(
            rule_category!(),
            else_clause.range(),
            markup! {
                "This "<Emphasis>"else"</Emphasis>" clause can be omitted because previous branches break early."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, if_stmt: &Self::State) -> Option<JsRuleAction> {
        let else_clause = if_stmt.else_clause()?;
        if let Some(stmts_list) = if_stmt.parent::<JsStatementList>() {
            let else_alternative = else_clause.alternate().ok()?;
            let if_pos = stmts_list
                .iter()
                .position(|x| x.syntax() == if_stmt.syntax())?;
            let else_token = else_clause.else_token().ok()?;
            let new_if_stmt = AnyJsStatement::from(if_stmt.clone().with_else_clause(None))
                .with_trailing_trivia_pieces(chain_trivia_pieces(
                    else_token.leading_trivia().pieces(),
                    trim_leading_trivia_pieces(else_token.trailing_trivia().pieces()),
                ))?;
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
            return Some(JsRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                markup! { "Omit the "<Emphasis>"else"</Emphasis>" clause." }.to_owned(),
                mutation,
            ));
        }
        None
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct ScopeMetadata {
    // We are inside a breakable structure
    is_breakable: bool,
}

fn breaks_early(statement: AnyJsStatement) -> Option<()> {
    let mut stmt_stack = vec![(statement, ScopeMetadata::default())];
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
