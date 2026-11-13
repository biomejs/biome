use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsStatement, AnyJsSwitchClause, JsBlockStatement, JsBreakStatement, JsContinueStatement,
    JsDebuggerStatement, JsDoWhileStatement, JsExpressionStatement, JsLabeledStatement, JsLanguage,
    JsReturnStatement, JsStatementList, JsSyntaxKind, JsSyntaxToken, JsThrowStatement,
    JsVariableStatement, T, TsDeclareFunctionDeclaration, TsImportEqualsDeclaration,
    TsTypeAliasDeclaration,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeExt, AstNodeList, BatchMutationExt};
use biome_rule_options::no_useless_lone_block_statements::NoUselessLoneBlockStatementsOptions;

declare_lint_rule! {
    /// Disallow unnecessary nested block statements.
    ///
    /// > In JavaScript, prior to ES6, standalone code blocks delimited by curly braces do not create a new scope and have no use.
    /// > In ES6, code blocks may create a new scope if a block-level binding (let and const), a class declaration or a function declaration (in strict mode) are present. A block is not considered redundant in these cases.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (foo) {
    ///   bar();
    ///   {
    ///     baz();
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// while (foo) {
    ///   bar();
    /// }
    /// ```
    ///
    pub NoUselessLoneBlockStatements {
        version: "1.3.3",
        name: "noUselessLoneBlockStatements",
        language: "js",
        sources: &[RuleSource::Eslint("no-lone-blocks").same()],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUselessLoneBlockStatements {
    type Query = Ast<JsBlockStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUselessLoneBlockStatementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let block = ctx.query();
        let file_source = ctx.source_type::<JsFileSource>();

        if JsLabeledStatement::can_cast(block.syntax().parent()?.kind()) {
            return None;
        }

        if AnyJsSwitchClause::can_cast(block.syntax().grand_parent()?.kind()) {
            return None;
        }

        if in_control_structure(block) {
            return None;
        }

        if block.statements().is_empty() {
            // Preserve empty blocks that contain comments
            if block.syntax().has_comments_descendants() {
                return None;
            }
            return Some(());
        }

        if block
            .statements()
            .iter()
            .any(|statement| statement_has_block_level_declaration(&statement, file_source))
        {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let block = ctx.query();
        let block_range = block.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                block_range,
                markup! {
                    "This block statement doesn't serve any purpose and can be safely removed."
                },
            )
                .note(markup! {
                "Standalone block statements without any block-level declarations are redundant in JavaScript and can be removed to simplify the code."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let block = ctx.query();
        let stmts_list = block.parent::<JsStatementList>()?;
        let block_pos = stmts_list
            .iter()
            .position(|x| x.syntax() == block.syntax())?;

        let inner_statements: Vec<AnyJsStatement> = block.statements().iter().collect();
        let inner_len = inner_statements.len();

        let prev_stmts = stmts_list.iter().take(block_pos);

        let next_stmts = stmts_list.iter().skip(block_pos + 1);

        let mut new_stmts: Vec<_> = prev_stmts
            .chain(inner_statements)
            .chain(next_stmts)
            .collect();

        // The braces of the removed block were separating the statements around them.
        // Terminate a statement that now ends right before another one, otherwise they
        // could be merged (e.g. `{ a() } b()` would become `a() b()`).
        let boundaries = [
            block_pos.checked_sub(1),
            (block_pos + inner_len).checked_sub(1),
        ];
        for index in boundaries.into_iter().flatten() {
            if index + 1 < new_stmts.len()
                && let Some(terminated) = with_terminating_semicolon(new_stmts[index].clone())
            {
                new_stmts[index] = terminated;
            }
        }

        let new_stmts_list = make::js_statement_list(new_stmts);

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(stmts_list, new_stmts_list);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove redundant block." }.to_owned(),
            mutation,
        ))
    }
}

fn statement_has_block_level_declaration(
    statement: &AnyJsStatement,
    file_source: &JsFileSource,
) -> bool {
    match statement {
        AnyJsStatement::JsVariableStatement(variable) => is_not_var_declaration(variable),
        AnyJsStatement::JsFunctionDeclaration(_) => file_source.is_module(),
        AnyJsStatement::JsClassDeclaration(_) => true,
        _ => false,
    }
}

/// Returns `statement` with a `;` added at its end, or `None` when it doesn't need one:
/// it already ends with a `;`, or it can't be merged with a following statement
/// (e.g. it ends with a block).
///
/// For statements with a nested body (`if`, loops, labels...), the `;` is added to the
/// innermost statement, because that's the one which would be merged.
fn with_terminating_semicolon(statement: AnyJsStatement) -> Option<AnyJsStatement> {
    macro_rules! terminate {
        ($node:ident, $kind:ident) => {
            if $node.semicolon_token().is_some() {
                None
            } else {
                add_semicolon($node, $kind::with_semicolon_token).map(AnyJsStatement::from)
            }
        };
    }
    macro_rules! terminate_body {
        ($node:ident) => {{
            let body = with_terminating_semicolon($node.body().ok()?)?;
            Some($node.with_body(body).into())
        }};
    }

    match statement {
        AnyJsStatement::JsBreakStatement(node) => terminate!(node, JsBreakStatement),
        AnyJsStatement::JsContinueStatement(node) => terminate!(node, JsContinueStatement),
        AnyJsStatement::JsDebuggerStatement(node) => terminate!(node, JsDebuggerStatement),
        AnyJsStatement::JsDoWhileStatement(node) => terminate!(node, JsDoWhileStatement),
        AnyJsStatement::JsExpressionStatement(node) => terminate!(node, JsExpressionStatement),
        AnyJsStatement::JsReturnStatement(node) => terminate!(node, JsReturnStatement),
        AnyJsStatement::JsThrowStatement(node) => terminate!(node, JsThrowStatement),
        AnyJsStatement::JsVariableStatement(node) => terminate!(node, JsVariableStatement),
        AnyJsStatement::TsDeclareFunctionDeclaration(node) => {
            terminate!(node, TsDeclareFunctionDeclaration)
        }
        AnyJsStatement::TsImportEqualsDeclaration(node) => {
            terminate!(node, TsImportEqualsDeclaration)
        }
        AnyJsStatement::TsTypeAliasDeclaration(node) => terminate!(node, TsTypeAliasDeclaration),
        AnyJsStatement::JsIfStatement(node) => match node.else_clause() {
            Some(else_clause) => {
                let alternate = with_terminating_semicolon(else_clause.alternate().ok()?)?;
                Some(
                    node.with_else_clause(Some(else_clause.with_alternate(alternate)))
                        .into(),
                )
            }
            None => {
                let consequent = with_terminating_semicolon(node.consequent().ok()?)?;
                Some(node.with_consequent(consequent).into())
            }
        },
        AnyJsStatement::JsForStatement(node) => terminate_body!(node),
        AnyJsStatement::JsForInStatement(node) => terminate_body!(node),
        AnyJsStatement::JsForOfStatement(node) => terminate_body!(node),
        AnyJsStatement::JsWhileStatement(node) => terminate_body!(node),
        AnyJsStatement::JsWithStatement(node) => terminate_body!(node),
        AnyJsStatement::JsLabeledStatement(node) => terminate_body!(node),
        _ => None,
    }
}

/// Sets a `;` token on `node`, moving the trailing trivia of its last token after the `;`.
fn add_semicolon<N>(
    node: N,
    with_semicolon: impl FnOnce(N, Option<JsSyntaxToken>) -> N,
) -> Option<N>
where
    N: AstNode<Language = JsLanguage>,
{
    let last_token = node.syntax().last_token()?;
    let semicolon =
        make::token(T![;]).with_trailing_trivia_pieces(last_token.trailing_trivia().pieces());
    let node = node.replace_token_discard_trivia(
        last_token.clone(),
        last_token.with_trailing_trivia_pieces([]),
    )?;
    Some(with_semicolon(node, Some(semicolon)))
}

fn in_control_structure(block: &JsBlockStatement) -> bool {
    if let Some(node) = block.syntax().parent() {
        let syntax_kind = node.kind();

        if let JsSyntaxKind::JS_ELSE_CLAUSE
        | JsSyntaxKind::JS_CATCH_CLAUSE
        | JsSyntaxKind::JS_FINALLY_CLAUSE = syntax_kind
        {
            return true;
        }
    }

    matches!(
        block.parent(),
        Some(
            AnyJsStatement::JsIfStatement(_)
                | AnyJsStatement::JsWhileStatement(_)
                | AnyJsStatement::JsDoWhileStatement(_)
                | AnyJsStatement::JsForStatement(_)
                | AnyJsStatement::JsForInStatement(_)
                | AnyJsStatement::JsForOfStatement(_)
                | AnyJsStatement::JsWithStatement(_)
                | AnyJsStatement::JsSwitchStatement(_)
                | AnyJsStatement::JsTryStatement(_)
                | AnyJsStatement::JsTryFinallyStatement(_)
        )
    )
}

fn is_not_var_declaration(variable: &JsVariableStatement) -> bool {
    variable
        .declaration().is_ok_and(|decl| !decl.is_var())
}
