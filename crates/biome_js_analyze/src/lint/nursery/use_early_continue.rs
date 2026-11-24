use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsArrowFunctionExpression, JsBlockStatement, JsBreakStatement,
    JsClassDeclaration, JsContinueStatement, JsDoWhileStatement, JsForInStatement,
    JsForOfStatement, JsForStatement, JsFunctionDeclaration, JsIdentifierAssignment,
    JsIdentifierBinding, JsIfStatement, JsLabeledStatement, JsLanguage, JsReferenceIdentifier,
    JsReturnStatement, JsStatementList, JsSyntaxToken, JsSyntaxTrivia, JsThrowStatement,
    JsUnaryOperator, JsVariableDeclaration, JsVariableKind, JsWhileStatement,
    JsxReferenceIdentifier, OperatorPrecedence, T, TsDeclareFunctionDeclaration,
    TsDeclareStatement, TsEnumDeclaration, TsExternalModuleDeclaration, TsGlobalDeclaration,
    TsImportEqualsDeclaration, TsInterfaceDeclaration, TsModuleDeclaration, TsTypeAliasDeclaration,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Direction, SyntaxKindSet,
    SyntaxTriviaPiece, TextRange, TokenText, TriviaPieceKind, WalkEvent, declare_node_union,
    trim_leading_trivia_pieces,
};
use biome_rule_options::use_early_continue::UseEarlyContinueOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Require an early `continue` instead of an `if` statement that wraps the rest of a loop body.
    ///
    /// When a loop body ends with an `if` statement that holds most of the work, the main code
    /// of the loop sits one level deeper than it needs to. Skipping the iterations you don't care
    /// about first, with `if (...) { continue; }`, keeps that code at the top level of the loop and
    /// makes it easier to follow.
    ///
    /// The rule reports an `if` statement without an `else` branch when it's the last statement
    /// of a loop body wrapped in braces, and it contains more statements than the
    /// `maximumStatements` option allows. Other statements can come before the `if` statement.
    /// The rule doesn't report an `if` statement whose last statement already leaves the iteration
    /// with `return`, `break`, `continue`, or `throw`, because an early `continue` wouldn't remove
    /// any nesting.
    ///
    /// The fix reverses the condition, adds a `continue` statement, and moves the content of the
    /// `if` statement after it. The fix isn't offered when:
    ///
    /// - the `if` statement contains a function, a class, a `using` declaration, or a TypeScript
    ///   declaration such as a type or an interface;
    /// - the `if` statement contains a `let` or `const` declaration, and either other statements
    ///   come before the `if` statement in the loop body, or the condition uses a variable with
    ///   the same name;
    /// - comments are attached to the braces of the `if` statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (const item of items) {
    ///     if (item.isActive) {
    ///         process(item);
    ///         save(item);
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// while (queue.length > 0) {
    ///     const item = queue.shift();
    ///     if (item.isActive) {
    ///         process(item);
    ///         save(item);
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// for (const item of items) {
    ///     if (!item.isActive) {
    ///         continue;
    ///     }
    ///     process(item);
    ///     save(item);
    /// }
    ///
    /// for (const item of items) {
    ///     if (item.isActive) {
    ///         process(item);
    ///     }
    /// }
    ///
    /// for (const item of items) {
    ///     if (item.isActive) {
    ///         process(item);
    ///         save(item);
    ///     }
    ///     finish(item);
    /// }
    ///
    /// for (const item of items) {
    ///     if (item.isActive) {
    ///         process(item);
    ///         save(item);
    ///     } else {
    ///         skip(item);
    ///     }
    /// }
    ///
    /// function findActive(items) {
    ///     for (const item of items) {
    ///         if (item.isActive) {
    ///             process(item);
    ///             return item;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `maximumStatements`
    ///
    /// Type: `number`
    ///
    /// Default: `1`
    ///
    /// The largest number of statements that an `if` statement at the end of a loop body can
    /// contain before the rule reports it. Empty statements (a lone `;`) aren't counted. Set it
    /// to `0` to report any `if` statement at the end of a loop body that contains at least one
    /// statement.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maximumStatements": 0
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// for (const item of items) {
    ///     if (item.isActive) {
    ///         process(item);
    ///     }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// for (const item of items) {
    ///     if (!item.isActive) {
    ///         continue;
    ///     }
    ///     process(item);
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - The [`noContinue`](https://biomejs.dev/linter/rules/no-continue/) rule enforces the
    ///   opposite style. Don't enable both rules.
    ///
    pub UseEarlyContinue {
        version: "next",
        name: "useEarlyContinue",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-continue").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseEarlyContinue {
    type Query = Ast<JsIfStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseEarlyContinueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.else_clause().is_some() || node.syntax().next_sibling().is_some() {
            return None;
        }
        let body = node
            .parent::<JsStatementList>()?
            .parent::<JsBlockStatement>()?;
        if !LOOP_KINDS.matches(body.syntax().parent()?.kind()) {
            return None;
        }

        let mut count = 0;
        let mut last = None;
        for statement in branch_statements(node.consequent().ok()?) {
            if statement.as_js_empty_statement().is_none() {
                count += 1;
                last = Some(statement);
            }
        }
        if count <= usize::from(ctx.options().maximum_statements())
            || last.is_some_and(|statement| EXIT_KINDS.matches(statement.syntax().kind()))
        {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let range = TextRange::new(
            node.if_token().ok()?.text_trimmed_range().start(),
            node.r_paren_token().ok()?.text_trimmed_range().end(),
        );
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"if"</Emphasis>" statement wraps the rest of the loop body."
                },
            )
            .note(markup! {
                "The main code of the loop is nested one level deeper than it needs to be, which makes it harder to follow."
            })
            .note(markup! {
                "Skip the iteration early with "<Emphasis>"continue"</Emphasis>" when the condition isn't met, then write the rest of the code after it."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let list = node.parent::<JsStatementList>()?;
        let test = node.test().ok()?;
        let consequent = node.consequent().ok()?;
        if !can_move_statements(
            branch_statements(consequent.clone()),
            &test,
            list.len() == 1,
        ) {
            return None;
        }

        let (guard_consequent, moved) = match consequent {
            AnyJsStatement::JsBlockStatement(block) => {
                let l_curly = block.l_curly_token().ok()?;
                let r_curly = block.r_curly_token().ok()?;
                // The braces are reused by the `if` statement that holds `continue`, so their
                // comments would no longer sit next to the code they describe.
                if l_curly.has_trailing_comments() || r_curly.has_leading_comments() {
                    return None;
                }
                let statements = block.statements();
                // `continue;` takes the place of the moved statements inside the braces, so it
                // gets the same spacing.
                let continue_token = make::token(T![continue]).with_leading_trivia_pieces(
                    whitespace_after_comments(
                        &statements.first()?.syntax().first_token()?.leading_trivia(),
                    ),
                );
                let semicolon_token = make::token(T![;]).with_trailing_trivia_pieces(
                    only_whitespace(&statements.last()?.syntax().last_token()?.trailing_trivia()),
                );
                let continue_statement = make::js_continue_statement(continue_token)
                    .with_semicolon_token(semicolon_token)
                    .build();
                let guard_block = make::js_block_statement(
                    l_curly,
                    make::js_statement_list([continue_statement.into()]),
                    r_curly,
                );
                (guard_block.into(), statements.iter().collect::<Vec<_>>())
            }
            statement => {
                let first_token = statement.syntax().first_token()?;
                // The moved statement goes on its own line, indented like the `if` statement.
                let indent = whitespace_after_comments(&node.if_token().ok()?.leading_trivia());
                let mut semicolon_token = make::token(T![;]);
                if !indent.iter().any(SyntaxTriviaPiece::is_newline) {
                    semicolon_token =
                        semicolon_token.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
                }
                let continue_statement = make::js_continue_statement(
                    make::token(T![continue]).with_leading_trivia_pieces(
                        whitespace_after_comments(&first_token.leading_trivia()),
                    ),
                )
                .with_semicolon_token(semicolon_token)
                .build();
                let leading_trivia: Vec<_> = indent
                    .into_iter()
                    .chain(trim_leading_trivia_pieces(
                        first_token.leading_trivia().pieces(),
                    ))
                    .collect();
                let moved = statement.with_leading_trivia_pieces(leading_trivia)?;
                (continue_statement.into(), vec![moved])
            }
        };

        let guard = node
            .clone()
            .with_test(negated_condition(test)?)
            .with_consequent(guard_consequent);
        let statements: Vec<_> = list
            .iter()
            .take(list.len() - 1)
            .chain([guard.into()])
            .chain(moved)
            .collect();

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(list, make::js_statement_list(statements));
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use an early "<Emphasis>"continue"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

const LOOP_KINDS: SyntaxKindSet<JsLanguage> = JsDoWhileStatement::KIND_SET
    .union(JsForInStatement::KIND_SET)
    .union(JsForOfStatement::KIND_SET)
    .union(JsForStatement::KIND_SET)
    .union(JsWhileStatement::KIND_SET);

/// Statements that leave the current loop iteration.
const EXIT_KINDS: SyntaxKindSet<JsLanguage> = JsReturnStatement::KIND_SET
    .union(JsBreakStatement::KIND_SET)
    .union(JsContinueStatement::KIND_SET)
    .union(JsThrowStatement::KIND_SET);

/// Declarations that can't be moved out of a block, because their scope or hoisting differs
/// between a block and the loop body around it.
const UNMOVABLE_DECLARATION_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionDeclaration::KIND_SET
    .union(JsClassDeclaration::KIND_SET)
    .union(TsDeclareFunctionDeclaration::KIND_SET)
    .union(TsDeclareStatement::KIND_SET)
    .union(TsEnumDeclaration::KIND_SET)
    .union(TsExternalModuleDeclaration::KIND_SET)
    .union(TsGlobalDeclaration::KIND_SET)
    .union(TsImportEqualsDeclaration::KIND_SET)
    .union(TsInterfaceDeclaration::KIND_SET)
    .union(TsModuleDeclaration::KIND_SET)
    .union(TsTypeAliasDeclaration::KIND_SET);

declare_node_union! {
    /// A use of a variable by name: a read, an assignment or update such as `x++`, or a JSX
    /// component name such as `<Item />`.
    pub AnyJsNameUse = JsReferenceIdentifier | JsIdentifierAssignment | JsxReferenceIdentifier
}

impl AnyJsNameUse {
    fn name_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsReferenceIdentifier(node) => node.value_token().ok(),
            Self::JsIdentifierAssignment(node) => node.name_token().ok(),
            Self::JsxReferenceIdentifier(node) => node.value_token().ok(),
        }
    }
}

/// Returns the statements of `statement` if it's a block, or `statement` itself otherwise.
fn branch_statements(statement: AnyJsStatement) -> impl Iterator<Item = AnyJsStatement> {
    let (block, single) = match statement {
        AnyJsStatement::JsBlockStatement(block) => (Some(block.statements()), None),
        statement => (None, Some(statement)),
    };
    block.into_iter().flat_map(|list| list.iter()).chain(single)
}

/// Returns `true` if `statements` can be moved from the `if` statement to the loop body without
/// changing which variable a name refers to.
///
/// A `let` or `const` declaration can only move when the `if` statement is the only statement of
/// the loop body, so that no earlier statement uses the same name, and when `test` doesn't use the
/// declared name either. `test` also can't call `eval`, because the code passed to it can use any
/// name.
fn can_move_statements(
    statements: impl Iterator<Item = AnyJsStatement>,
    test: &AnyJsExpression,
    is_only_statement: bool,
) -> bool {
    let mut lexical_names = Vec::new();
    for statement in statements {
        match statement {
            AnyJsStatement::JsVariableStatement(statement) => {
                let Ok(declaration) = statement.declaration() else {
                    return false;
                };
                match declaration.variable_kind() {
                    Ok(JsVariableKind::Var) => {}
                    Ok(JsVariableKind::Let | JsVariableKind::Const) => {
                        lexical_names.extend(declared_names(&declaration));
                    }
                    Ok(JsVariableKind::Using) | Err(_) => return false,
                }
            }
            AnyJsStatement::JsLabeledStatement(statement) => {
                if labels_function_declaration(&statement) {
                    return false;
                }
            }
            statement => {
                if UNMOVABLE_DECLARATION_KINDS.matches(statement.syntax().kind()) {
                    return false;
                }
            }
        }
    }
    if lexical_names.is_empty() {
        return true;
    }
    if !is_only_statement {
        return false;
    }
    !test
        .syntax()
        .preorder_of_kinds(AnyJsNameUse::KIND_SET)
        .filter_map(|event| match event {
            WalkEvent::Enter(node) => AnyJsNameUse::cast(node)?.name_token(),
            WalkEvent::Leave(_) => None,
        })
        .any(|token| {
            let name = token.text_trimmed();
            name == "eval" || lexical_names.iter().any(|declared| declared.text() == name)
        })
}

/// Returns the names of the variables declared by `declaration`, including the names in
/// destructuring patterns.
fn declared_names(declaration: &JsVariableDeclaration) -> impl Iterator<Item = TokenText> {
    declaration
        .declarators()
        .iter()
        .filter_map(|declarator| declarator.ok()?.id().ok())
        .flat_map(|id| {
            id.syntax()
                .preorder_of_kinds(JsIdentifierBinding::KIND_SET)
                .filter_map(|event| match event {
                    WalkEvent::Enter(node) => Some(
                        JsIdentifierBinding::cast(node)?
                            .name_token()
                            .ok()?
                            .token_text_trimmed(),
                    ),
                    WalkEvent::Leave(_) => None,
                })
                .collect::<Vec<_>>()
        })
}

/// Returns `true` if `statement` labels a function declaration, as in `label: function f() {}`.
fn labels_function_declaration(statement: &JsLabeledStatement) -> bool {
    let mut body = statement.body();
    while let Ok(AnyJsStatement::JsLabeledStatement(statement)) = body {
        body = statement.body();
    }
    matches!(body, Ok(AnyJsStatement::JsFunctionDeclaration(_)))
}

/// Returns the opposite of `test`: the operand of `!operand`, or `test` negated with `!`.
fn negated_condition(test: AnyJsExpression) -> Option<AnyJsExpression> {
    if let AnyJsExpression::JsUnaryExpression(unary) = &test
        && unary.operator().ok()? == JsUnaryOperator::LogicalNot
    {
        let operand = unary.argument().ok()?.omit_parentheses();
        let kept_range = operand.syntax().text_range_with_trivia();
        let has_removed_comments = test
            .syntax()
            .descendants_tokens(Direction::Next)
            .filter(|token| !kept_range.contains_range(token.text_range()))
            .any(|token| token.has_leading_comments() || token.has_trailing_comments());
        if !has_removed_comments {
            return Some(operand);
        }
    }

    let first_token = test.syntax().first_token()?;
    let last_token = test.syntax().last_token()?;
    let test = test.trim_trivia()?;
    let operand = if test.precedence().ok()? < OperatorPrecedence::Unary
        || JsArrowFunctionExpression::can_cast(test.syntax().kind())
    {
        make::parenthesized(test).into()
    } else {
        test
    };
    AnyJsExpression::from(make::js_unary_expression(
        make::token(T![!]).with_leading_trivia_pieces(first_token.leading_trivia().pieces()),
        operand,
    ))
    .with_trailing_trivia_pieces(last_token.trailing_trivia().pieces())
}

/// Returns the pieces of `trivia` that follow its last comment, which hold the line break and
/// indentation before the token.
fn whitespace_after_comments(trivia: &JsSyntaxTrivia) -> Vec<SyntaxTriviaPiece<JsLanguage>> {
    let mut pieces: Vec<_> = trivia.pieces().collect();
    let start = pieces
        .iter()
        .rposition(SyntaxTriviaPiece::is_comments)
        .map_or(0, |index| index + 1);
    pieces.split_off(start)
}

/// Returns the pieces of `trivia` if it only contains whitespace, or nothing otherwise. The
/// whitespace before a comment belongs to the comment.
fn only_whitespace(trivia: &JsSyntaxTrivia) -> Vec<SyntaxTriviaPiece<JsLanguage>> {
    if trivia.pieces().any(|piece| piece.is_comments()) {
        return Vec::new();
    }
    trivia.pieces().collect()
}
