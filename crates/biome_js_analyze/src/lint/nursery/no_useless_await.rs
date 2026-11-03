use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsLiteralExpression, JsArrayExpression, JsArrowFunctionExpression, JsAwaitExpression,
    JsBinaryExpression, JsBlockStatement, JsClassExpression, JsElseClause, JsExpressionStatement,
    JsFunctionBody, JsFunctionExpression, JsIfStatement, JsInExpression, JsInitializerClause,
    JsInstanceofExpression, JsLanguage, JsModule, JsModuleItemList, JsParenthesizedExpression,
    JsPostUpdateExpression, JsPreUpdateExpression, JsReturnStatement, JsScript, JsStatementList,
    JsSyntaxKind, JsThrowStatement, JsUnaryExpression, JsVariableDeclaration,
    JsVariableDeclarator, JsVariableDeclaratorList, JsVariableStatement, JsxTagExpression, T,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, BatchMutationExt, Direction, SyntaxKindSet};
use biome_rule_options::no_useless_await::NoUselessAwaitOptions;

use crate::{JsRuleAction, ast_utils::starts_unterminated_statement};

declare_lint_rule! {
    /// Disallow `await` on values that are never promises.
    ///
    /// The `await` keyword waits for a promise to settle and returns its result.
    /// Values such as numbers, strings, arrays, functions, and the results of
    /// comparisons are never promises, so awaiting them returns the same value.
    /// The `await` still pauses the function until other pending work runs,
    /// which makes the code slower and suggests that the value is a promise when it isn't.
    ///
    /// Awaiting an array doesn't wait for the promises inside it.
    /// Use `Promise.all()` or `Promise.allSettled()` to wait for all of them.
    ///
    /// The rule only reports values that can be identified as non-promises from the code alone,
    /// such as literals, arrays, functions, classes, JSX, and the results of
    /// arithmetic, comparison, and unary operators.
    ///
    /// The fix is unsafe because removing `await` makes the code that follows it run earlier.
    /// Biome offers the fix only when the `await` is the last thing its function or module runs.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// await 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await [fetchUser(), fetchPosts()];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// async function check(a, b) {
    ///     return await (a === b);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// await fetchUser();
    /// await Promise.all([fetchUser(), fetchPosts()]);
    /// await (condition ? fetchUser() : fetchPosts());
    /// await new Promise((resolve) => setTimeout(resolve, 100));
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to report `await` on any value that isn't a promise, using type information, see [`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/).
    ///
    pub NoUselessAwait {
        version: "next",
        name: "noUselessAwait",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("no-unnecessary-await").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessAwait {
    type Query = Ast<JsAwaitExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUselessAwaitOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let value = ctx.query().argument().ok()?.inner_expression()?;
        let is_untagged_template = value
            .as_js_template_expression()
            .is_some_and(|template| template.tag().is_none());
        (is_untagged_template || NON_PROMISE_KINDS.matches(value.syntax().kind())).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.await_token().ok()?.text_trimmed_range(),
                markup! {
                    "This value is never a promise, so "<Emphasis>"await"</Emphasis>" is unnecessary."
                },
            )
            .note(markup! {
                "Awaiting a value that isn't a promise returns the same value, but still delays the code that follows it."
            })
            .note(markup! {
                "Remove "<Emphasis>"await"</Emphasis>". To wait for several promises, pass them to "<Emphasis>"Promise.all()"</Emphasis>" instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let expression = AnyJsExpression::from(node.clone());
        let outer = expression.outer_expression()?;
        if !is_last_evaluated(&outer) {
            return None;
        }

        let await_token = node.await_token().ok()?;
        let argument = node.argument().ok()?;
        let first_token = argument.syntax().first_token()?;
        if outer.syntax().first_token().as_ref() == Some(&await_token) {
            // Without `await`, the value could join the previous statement, or a leading `{`,
            // `function`, or `class` would start a block or a declaration instead of an
            // expression.
            if outer.parent::<JsExpressionStatement>().is_some()
                && (starts_unterminated_statement(&expression)
                    || STATEMENT_START_TOKENS.contains(first_token.kind()))
            {
                return None;
            }
            // An arrow function body that starts with `{` is a block.
            if outer.parent::<JsArrowFunctionExpression>().is_some()
                && first_token.kind() == T!['{']
            {
                return None;
            }
        }

        // A line break after `return` or `throw` ends the statement, so a value that starts on
        // the next line needs parentheses to stay part of it.
        let replacement = if first_token.has_leading_newline()
            && node.parent::<JsParenthesizedExpression>().is_none()
        {
            let trailing_trivia = argument.syntax().last_token()?.trailing_trivia();
            make::js_parenthesized_expression(
                make::token(T!['('])
                    .with_leading_trivia_pieces(await_token.leading_trivia().pieces())
                    .with_trailing_trivia_pieces(await_token.trailing_trivia().pieces()),
                argument.with_trailing_trivia_pieces([])?,
                make::token(T![')']).with_trailing_trivia_pieces(trailing_trivia.pieces()),
            )
            .into()
        } else {
            let leading_trivia: Vec<_> = await_token
                .leading_trivia()
                .pieces()
                .chain(
                    await_token
                        .trailing_trivia()
                        .pieces()
                        .skip_while(|piece| piece.is_whitespace()),
                )
                .collect();
            argument.prepend_trivia_pieces(leading_trivia)?
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(expression, replacement);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove "<Emphasis>"await"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` if nothing in the enclosing function or module runs after `expression`.
///
/// Removing an `await` makes the code after it run before other pending work, so the fix is only
/// offered when there is no such code.
fn is_last_evaluated(expression: &AnyJsExpression) -> bool {
    let mut current = expression.syntax().clone();
    while let Some(parent) = current.parent() {
        let kind = parent.kind();
        if ROOT_KINDS.matches(kind) {
            return true;
        }
        let is_last_evaluated_child = if LIST_KINDS.matches(kind) {
            // A `using` declaration earlier in the scope runs its disposal code when the scope ends.
            current.next_sibling().is_none()
                && !current.siblings(Direction::Prev).any(|sibling| {
                    JsVariableStatement::cast(sibling)
                        .and_then(|statement| statement.declaration().ok())
                        .is_some_and(|declaration| is_using_declaration(&declaration))
                })
        } else if let Some(statement) = JsIfStatement::cast_ref(&parent) {
            statement
                .test()
                .is_ok_and(|test| test.syntax() != &current)
        } else if let Some(declaration) = JsVariableDeclaration::cast_ref(&parent) {
            !is_using_declaration(&declaration)
        } else if let Some(declarator) = JsVariableDeclarator::cast_ref(&parent) {
            // Destructuring runs default values and iterators after the value is available.
            declarator
                .id()
                .is_ok_and(|id| matches!(id, AnyJsBindingPattern::AnyJsBinding(_)))
        } else {
            PASS_THROUGH_KINDS.matches(kind)
        };
        if !is_last_evaluated_child {
            return false;
        }
        current = parent;
    }
    false
}

/// Returns `true` for `using` and `await using` declarations, which run disposal code when the
/// enclosing scope ends.
fn is_using_declaration(declaration: &JsVariableDeclaration) -> bool {
    !(declaration.is_const() || declaration.is_let() || declaration.is_var())
}

/// Expressions whose result is never a promise.
const NON_PROMISE_KINDS: SyntaxKindSet<JsLanguage> = JsArrayExpression::KIND_SET
    .union(JsArrowFunctionExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsBinaryExpression::KIND_SET)
    .union(JsInExpression::KIND_SET)
    .union(JsInstanceofExpression::KIND_SET)
    .union(JsClassExpression::KIND_SET)
    .union(JsFunctionExpression::KIND_SET)
    .union(JsxTagExpression::KIND_SET)
    .union(JsUnaryExpression::KIND_SET)
    .union(JsPreUpdateExpression::KIND_SET)
    .union(JsPostUpdateExpression::KIND_SET)
    .union(AnyJsLiteralExpression::KIND_SET);

/// Nodes whose body runs last in a function or module: once the last statement of the body
/// finishes, nothing else runs.
const ROOT_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionBody::KIND_SET
    .union(JsModule::KIND_SET)
    .union(JsScript::KIND_SET)
    .union(JsArrowFunctionExpression::KIND_SET);

/// Statement lists, where only the last item runs last.
const LIST_KINDS: SyntaxKindSet<JsLanguage> = JsStatementList::KIND_SET
    .union(JsModuleItemList::KIND_SET)
    .union(JsVariableDeclaratorList::KIND_SET);

/// Nodes that run nothing after their child that contains the `await`.
const PASS_THROUGH_KINDS: SyntaxKindSet<JsLanguage> = JsBlockStatement::KIND_SET
    .union(JsElseClause::KIND_SET)
    .union(JsExpressionStatement::KIND_SET)
    .union(JsReturnStatement::KIND_SET)
    .union(JsThrowStatement::KIND_SET)
    .union(JsVariableStatement::KIND_SET)
    .union(JsInitializerClause::KIND_SET);

/// Tokens that make a statement a block or a declaration instead of an expression.
const STATEMENT_START_TOKENS: TokenSet<JsSyntaxKind> = token_set![
    T!['{'],
    T![function],
    T![class],
    T![async],
    T![let],
    T![@],
];
