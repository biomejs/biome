use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsStatement, JsDoWhileStatement, JsElseClause, JsForInStatement, JsForOfStatement,
    JsForStatement, JsIfStatement, JsLanguage, JsSyntaxTrivia, JsWhileStatement, JsWithStatement,
    TriviaPieceKind, T,
};

use biome_rowan::{declare_node_union, AstNode, BatchMutationExt, SyntaxTriviaPiece};

use crate::JsRuleAction;
use crate::{use_block_statements_diagnostic, use_block_statements_replace_body};

declare_lint_rule! {
    /// Requires following curly brace conventions.
    ///
    /// JavaScript allows the omission of curly braces when a block contains only one statement. However, it is considered by many to be best practice to never omit curly braces around blocks, even when they are optional, because it can lead to bugs and reduces code clarity.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    ///  if (x) x;
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///  if (x) {
    ///    x;
    ///  } else y;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (x) {
    ///   x;
    /// } else if (y) y;
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///    for (;;);
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///    for (p in obj);
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   for (x of xs);
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   do;
    ///   while (x);
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///    while (x);
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   with (x);
    /// ```
    pub UseBlockStatements {
        version: "1.0.0",
        name: "useBlockStatements",
        language: "js",
        sources: &[RuleSource::Eslint("curly")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyJsBlockStatement = JsIfStatement | JsElseClause | JsDoWhileStatement | JsForInStatement | JsForOfStatement | JsForStatement | JsWhileStatement | JsWithStatement
}

impl Rule for UseBlockStatements {
    type Query = Ast<AnyJsBlockStatement>;
    type State = UseBlockStatementsOperationType;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            AnyJsBlockStatement::JsIfStatement(stmt) => {
                use_block_statements_diagnostic!(stmt, consequent)
            }
            AnyJsBlockStatement::JsDoWhileStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            AnyJsBlockStatement::JsForInStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            AnyJsBlockStatement::JsForOfStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            AnyJsBlockStatement::JsForStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            AnyJsBlockStatement::JsWhileStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            AnyJsBlockStatement::JsWithStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            AnyJsBlockStatement::JsElseClause(stmt) => {
                let body = stmt.alternate().ok()?;
                if matches!(body, AnyJsStatement::JsEmptyStatement(_)) {
                    return Some(UseBlockStatementsOperationType::ReplaceBody);
                }
                let is_block = matches!(
                    body,
                    AnyJsStatement::JsBlockStatement(_) | AnyJsStatement::JsIfStatement(_)
                );
                if !is_block {
                    return Some(UseBlockStatementsOperationType::Wrap(body));
                }
                None
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Block statements are preferred in this position."
            },
        ))
    }

    fn action(
        ctx: &RuleContext<Self>,
        nodes_need_to_replaced: &Self::State,
    ) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match nodes_need_to_replaced {
            UseBlockStatementsOperationType::Wrap(stmt) => {
                let mut l_curly_token = make::token(T!['{']);
                let r_curly_token = make::token(T!['}']);

                // Ensure the opening curly token is separated from the previous token by at least one space
                let has_previous_space = stmt
                    .syntax()
                    .first_token()
                    .and_then(|token| token.prev_token())
                    .is_some_and(|token| {
                        token
                            .trailing_trivia()
                            .pieces()
                            .rev()
                            .take_while(|piece| !piece.is_newline())
                            .any(|piece| piece.is_whitespace())
                    });

                if !has_previous_space {
                    l_curly_token =
                        l_curly_token.with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]);
                }

                // Clone the leading trivia of the single statement as the
                // leading trivia of the closing curly token
                let mut leading_trivia = stmt
                    .syntax()
                    .first_leading_trivia()
                    .as_ref()
                    .map(collect_to_first_newline)
                    .unwrap_or_default();

                // If the statement has no leading trivia, add a space after
                // the opening curly token
                if leading_trivia.is_empty() {
                    l_curly_token =
                        l_curly_token.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
                }

                // If the leading trivia for the statement contains any newline,
                // then the indentation is probably one level too deep for the
                // closing curly token, clone the leading trivia from the
                // parent node instead
                if leading_trivia.iter().any(|piece| piece.is_newline()) {
                    // Find the parent block statement node, skipping over
                    // else-clause nodes if this statement is part of an
                    // else-if chain
                    let mut node = node.clone();
                    while let Some(parent) = node.parent::<AnyJsBlockStatement>() {
                        if !matches!(parent, AnyJsBlockStatement::JsElseClause(_)) {
                            break;
                        }

                        node = parent;
                    }

                    leading_trivia = node
                        .syntax()
                        .first_leading_trivia()
                        .as_ref()
                        .map(collect_to_first_newline)
                        .unwrap_or_default();
                }

                // Apply the cloned trivia to the closing curly token, or
                // fallback to a single space if it's still empty
                let r_curly_token = if !leading_trivia.is_empty() {
                    let leading_trivia = leading_trivia
                        .iter()
                        .rev()
                        .map(|piece| (piece.kind(), piece.text()));

                    r_curly_token.with_leading_trivia(leading_trivia)
                } else {
                    let has_trailing_single_line_comments =
                        stmt.syntax().last_trailing_trivia().is_some_and(|trivia| {
                            trivia
                                .pieces()
                                .any(|trivia| trivia.kind() == TriviaPieceKind::SingleLineComment)
                        });
                    // if the node we have to enclose has some trailing comments, then we add a new line
                    // to the leading trivia of the right curly brace
                    if !has_trailing_single_line_comments {
                        r_curly_token.with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                    } else {
                        r_curly_token.with_leading_trivia([(TriviaPieceKind::Newline, "\n")])
                    }
                };

                mutation.replace_node_discard_trivia(
                    stmt.clone(),
                    AnyJsStatement::JsBlockStatement(make::js_block_statement(
                        l_curly_token,
                        make::js_statement_list([stmt.clone()]),
                        r_curly_token,
                    )),
                );
            }
            UseBlockStatementsOperationType::ReplaceBody => match node {
                AnyJsBlockStatement::JsIfStatement(stmt) => {
                    use_block_statements_replace_body!(
                        JsIfStatement,
                        with_consequent,
                        mutation,
                        node,
                        stmt
                    )
                }
                AnyJsBlockStatement::JsElseClause(stmt) => {
                    use_block_statements_replace_body!(
                        JsElseClause,
                        with_alternate,
                        mutation,
                        node,
                        stmt
                    )
                }
                AnyJsBlockStatement::JsDoWhileStatement(stmt) => {
                    use_block_statements_replace_body!(JsDoWhileStatement, mutation, node, stmt)
                }
                AnyJsBlockStatement::JsForInStatement(stmt) => {
                    use_block_statements_replace_body!(JsForInStatement, mutation, node, stmt)
                }
                AnyJsBlockStatement::JsForOfStatement(stmt) => {
                    use_block_statements_replace_body!(JsForOfStatement, mutation, node, stmt)
                }
                AnyJsBlockStatement::JsForStatement(stmt) => {
                    use_block_statements_replace_body!(JsForStatement, mutation, node, stmt)
                }
                AnyJsBlockStatement::JsWhileStatement(stmt) => {
                    use_block_statements_replace_body!(JsWhileStatement, mutation, node, stmt)
                }
                AnyJsBlockStatement::JsWithStatement(stmt) => {
                    use_block_statements_replace_body!(JsWithStatement, mutation, node, stmt)
                }
            },
        };
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Wrap the statement with a `JsBlockStatement`" }.to_owned(),
            mutation,
        ))
    }
}

/// Collect newline and comment trivia pieces in reverse order up to the first newline included
fn collect_to_first_newline(trivia: &JsSyntaxTrivia) -> Vec<SyntaxTriviaPiece<JsLanguage>> {
    let mut has_newline = false;
    trivia
        .pieces()
        .rev()
        .filter(|piece| piece.is_newline() || piece.is_whitespace())
        .take_while(|piece| {
            let had_newline = has_newline;
            has_newline |= piece.is_newline();
            !had_newline
        })
        .collect()
}

pub enum UseBlockStatementsOperationType {
    Wrap(AnyJsStatement),
    ReplaceBody,
}

#[macro_export]
macro_rules! use_block_statements_diagnostic {
    ($id:ident, $field:ident) => {{
        let body = $id.$field().ok()?;
        if matches!(body, AnyJsStatement::JsEmptyStatement(_)) {
            Some(UseBlockStatementsOperationType::ReplaceBody)
        } else if !matches!(body, AnyJsStatement::JsBlockStatement(_)) {
            Some(UseBlockStatementsOperationType::Wrap(body))
        } else {
            None
        }
    }};
    ($id:ident) => {
        use_block_statements_diagnostic!($id, body)
    };
}

#[macro_export]
macro_rules! use_block_statements_replace_body {
    ($stmt_type:ident, $builder_method:ident, $mutation:ident, $node:ident, $stmt:ident) => {
        $mutation.replace_node(
            $node.clone(),
            AnyJsBlockStatement::$stmt_type($stmt.clone().$builder_method(
                AnyJsStatement::JsBlockStatement(make::js_block_statement(
                    make::token(T!['{']).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    make::js_statement_list([]),
                    make::token(T!['}']),
                )),
            )),
        )
    };

    ($stmt_type:ident, $mutation:ident, $node:ident, $stmt:ident) => {
        use_block_statements_replace_body!($stmt_type, with_body, $mutation, $node, $stmt)
    };
}
