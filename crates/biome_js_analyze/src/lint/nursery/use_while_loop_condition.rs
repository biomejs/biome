use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, AnyJsIdentifierReference, AnyJsStatement, JsBlockStatement, JsBreakStatement,
    JsClassDeclaration, JsDoWhileStatement, JsForInStatement, JsForOfStatement, JsForStatement,
    JsFunctionDeclaration, JsIfStatement, JsLabeledStatement, JsLanguage, JsSwitchStatement,
    JsSyntaxToken, JsUnaryOperator, JsWhileStatement, OperatorPrecedence, T,
    binding_ext::AnyJsBindingDeclaration, static_value::StaticValue,
};
use biome_rowan::{
    AstNode, AstNodeList, BatchMutationExt, Direction, SyntaxKindSet, SyntaxResult, WalkEvent,
    declare_node_union, trim_leading_trivia_pieces,
};
use biome_rule_options::use_while_loop_condition::UseWhileLoopConditionOptions;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require the condition that ends a loop to be written in the `while` statement.
    ///
    /// A loop such as `while (true)` or `for (;;)` that starts its body with
    /// `if (condition) { break; }` splits its stopping condition across two places.
    /// Writing the condition in the `while` statement shows when the loop stops
    /// without having to read its body.
    ///
    /// The rule reports `while (true)`, `do ... while (true)`, `for (;;)`, and
    /// `for (; true;)` loops whose body starts with an `if` statement that contains
    /// only a `break`. The rule ignores loops that have a label, and loops that can
    /// also be stopped by another `break` in their body.
    ///
    /// The fix isn't offered when comments in the loop header or in the `if`
    /// statement would be lost.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// while (true) {
    ///     if (!hasMore()) {
    ///         break;
    ///     }
    ///     processNext();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (;;) {
    ///     if (done) break;
    ///     processNext();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// do {
    ///     if (!hasMore()) {
    ///         break;
    ///     }
    ///     processNext();
    /// } while (true);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// while (hasMore()) {
    ///     processNext();
    /// }
    ///
    /// while (!done) {
    ///     processNext();
    /// }
    ///
    /// while (true) {
    ///     prepare();
    ///     if (!hasMore()) {
    ///         break;
    ///     }
    /// }
    ///
    /// while (true) {
    ///     if (!hasMore()) {
    ///         break;
    ///     }
    ///     if (cancelled) {
    ///         break;
    ///     }
    /// }
    /// ```
    ///
    pub UseWhileLoopCondition {
        version: "2.6.4",
        name: "useWhileLoopCondition",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-while-loop-condition").same()],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnyJsLoopStatement = JsDoWhileStatement | JsForStatement | JsWhileStatement
}

impl Rule for UseWhileLoopCondition {
    type Query = Semantic<AnyJsLoopStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseWhileLoopConditionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !node.is_infinite() || node.parent::<JsLabeledStatement>().is_some() {
            return None;
        }

        let body = node.block_body()?;
        let mut statements = body.statements().iter();
        let guard = statements.next()?;
        let guard = guard.as_js_if_statement()?;
        if guard.else_clause().is_some() || !is_only_unlabeled_break(&guard.consequent().ok()?) {
            return None;
        }

        if statements.any(|statement| has_unlabeled_break(&statement))
            || references_body_declaration(ctx.model(), &body, guard)
        {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let guard = ctx.query().guard()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                guard.range(),
                markup! {
                    "The condition that ends this loop is checked by an "<Emphasis>"if"</Emphasis>" statement at the start of its body."
                },
            )
            .note(markup! {
                "Writing the condition in the "<Emphasis>"while"</Emphasis>" statement shows when the loop stops without reading its body."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let body = node.block_body()?;
        let guard = node.guard()?;
        if has_removed_comments(node, &body, &guard) {
            return None;
        }

        let (while_token, l_paren_token, r_paren_token) = match node {
            AnyJsLoopStatement::JsWhileStatement(node) => (
                node.while_token().ok()?,
                node.l_paren_token().ok()?,
                node.r_paren_token().ok()?,
            ),
            AnyJsLoopStatement::JsForStatement(node) => (
                make::token(T![while])
                    .with_trailing_trivia_pieces(node.for_token().ok()?.trailing_trivia().pieces()),
                node.l_paren_token().ok()?.with_trailing_trivia([]),
                node.r_paren_token().ok()?,
            ),
            AnyJsLoopStatement::JsDoWhileStatement(node) => (
                make::token_with_trailing_space(T![while]),
                make::token(T!['(']),
                make::token(T![')'])
                    .with_trailing_trivia_pieces(node.do_token().ok()?.trailing_trivia().pieces()),
            ),
        };

        // The removed `if` statement's leading trivia holds the line break and
        // indentation of the first line in the body, so the next statement or
        // comment takes it over.
        let guard_leading_trivia = guard.syntax().first_token()?.leading_trivia();
        let mut statements: Vec<_> = body.statements().iter().skip(1).collect();
        let mut r_curly_token = body.r_curly_token().ok()?;
        if let Some(first) = statements.first_mut() {
            let leading_trivia = first.syntax().first_token()?.leading_trivia();
            *first = first
                .clone()
                .with_leading_trivia_pieces(trim_leading_trivia_pieces(leading_trivia.pieces()))?
                .prepend_trivia_pieces(guard_leading_trivia.pieces())?;
        } else if r_curly_token.has_leading_comments() {
            let leading_trivia = r_curly_token.leading_trivia();
            let trivia: Vec<_> = guard_leading_trivia
                .pieces()
                .chain(trim_leading_trivia_pieces(leading_trivia.pieces()))
                .collect();
            r_curly_token = r_curly_token.with_leading_trivia_pieces(trivia);
        } else {
            r_curly_token = r_curly_token.with_leading_trivia([]);
        }

        let new_body = make::js_block_statement(
            body.l_curly_token().ok()?,
            make::js_statement_list(statements),
            r_curly_token,
        );
        let new_loop = make::js_while_statement(
            while_token,
            l_paren_token,
            loop_condition(guard.test().ok()?)?,
            r_paren_token,
            new_body.into(),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsStatement::cast(node.syntax().clone())?,
            new_loop.into(),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Move the condition into the "<Emphasis>"while"</Emphasis>" statement." }
                .to_owned(),
            mutation,
        ))
    }
}

impl AnyJsLoopStatement {
    /// Returns `true` for `while (true)`, `do ... while (true)`, `for (;;)`, and
    /// `for (; true;)`.
    fn is_infinite(&self) -> bool {
        match self {
            Self::JsWhileStatement(node) => node.test().is_ok_and(is_true_literal),
            Self::JsDoWhileStatement(node) => node.test().is_ok_and(is_true_literal),
            Self::JsForStatement(node) => {
                node.initializer().is_none()
                    && node.update().is_none()
                    && node.test().is_none_or(is_true_literal)
            }
        }
    }

    fn body(&self) -> SyntaxResult<AnyJsStatement> {
        match self {
            Self::JsWhileStatement(node) => node.body(),
            Self::JsDoWhileStatement(node) => node.body(),
            Self::JsForStatement(node) => node.body(),
        }
    }

    fn block_body(&self) -> Option<JsBlockStatement> {
        JsBlockStatement::cast(self.body().ok()?.into_syntax())
    }

    /// Returns the `if` statement at the start of the loop body.
    fn guard(&self) -> Option<JsIfStatement> {
        JsIfStatement::cast(self.block_body()?.statements().first()?.into_syntax())
    }
}

/// Kinds whose `break` statements can't stop the enclosing loop: nested loops and
/// `switch` statements have their own `break` target, and functions and classes
/// start a new scope that `break` can't leave.
/// Expressions are included because they can only contain a `break` inside a
/// function or class.
const BREAK_BOUNDARY_KINDS: SyntaxKindSet<JsLanguage> = AnyJsExpression::KIND_SET
    .union(JsFunctionDeclaration::KIND_SET)
    .union(JsClassDeclaration::KIND_SET)
    .union(JsSwitchStatement::KIND_SET)
    .union(JsDoWhileStatement::KIND_SET)
    .union(JsForStatement::KIND_SET)
    .union(JsForInStatement::KIND_SET)
    .union(JsForOfStatement::KIND_SET)
    .union(JsWhileStatement::KIND_SET);

fn is_true_literal(expression: AnyJsExpression) -> bool {
    matches!(
        expression.omit_parentheses().as_static_value(),
        Some(StaticValue::Boolean(token)) if token.text_trimmed() == "true"
    )
}

/// Returns `true` if `statement` is `break;` or `{ break; }`.
fn is_only_unlabeled_break(statement: &AnyJsStatement) -> bool {
    match statement {
        AnyJsStatement::JsBreakStatement(statement) => statement.label().is_none(),
        AnyJsStatement::JsBlockStatement(block) => {
            let statements = block.statements();
            statements.len() == 1
                && matches!(
                    statements.first(),
                    Some(AnyJsStatement::JsBreakStatement(statement)) if statement.label().is_none()
                )
        }
        _ => false,
    }
}

/// Returns `true` if `statement` contains a `break` without a label that stops
/// the loop around it.
fn has_unlabeled_break(statement: &AnyJsStatement) -> bool {
    let mut events = statement.syntax().preorder();
    while let Some(event) = events.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if let Some(statement) = JsBreakStatement::cast_ref(&node) {
            if statement.label().is_none() {
                return true;
            }
        } else if BREAK_BOUNDARY_KINDS.matches(node.kind()) {
            events.skip_subtree();
        }
    }
    false
}

/// Returns `true` if the condition of `guard` uses a variable declared in the loop
/// body with `let`, `const`, `class`, or `function`. Moving the condition out of
/// the body would make it refer to a different variable, or to none at all.
fn references_body_declaration(
    model: &SemanticModel,
    body: &JsBlockStatement,
    guard: &JsIfStatement,
) -> bool {
    let Ok(test) = guard.test() else {
        return true;
    };
    let body_range = body.range();
    let test_range = test.range();
    test.syntax()
        .descendants()
        .filter_map(AnyJsIdentifierReference::cast)
        .filter_map(|reference| model.binding(&reference))
        .any(|binding| {
            let range = binding.tree().range();
            body_range.contains_range(range)
                && !test_range.contains_range(range)
                && !is_var_binding(&binding)
        })
}

fn is_var_binding(binding: &Binding) -> bool {
    let Some(declaration) = binding.tree().declaration() else {
        return false;
    };
    let declaration = declaration
        .parent_binding_pattern_declaration()
        .unwrap_or(declaration);
    matches!(
        declaration,
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator)
            if declarator.declaration().is_some_and(|declaration| declaration.is_var())
    )
}

/// Returns `true` if the fix would delete a comment.
///
/// The fix rebuilds the loop header and removes the `if` statement, so comments
/// there would be lost. Comments before the loop, after the loop, and in the
/// statements that stay in the body are kept.
fn has_removed_comments(
    node: &AnyJsLoopStatement,
    body: &JsBlockStatement,
    guard: &JsIfStatement,
) -> bool {
    if !node.syntax().has_comments_descendants() {
        return false;
    }
    let first = node.syntax().first_token();
    let last = node.syntax().last_token();
    let Ok(r_curly) = body.r_curly_token() else {
        return true;
    };
    let kept_start = guard.syntax().text_range_with_trivia().end();
    let kept_end = r_curly.text_trimmed_range().start();
    let is_kept = |token: &JsSyntaxToken| {
        let range = token.text_range();
        range.start() >= kept_start && range.end() <= kept_end
    };
    node.syntax()
        .descendants_tokens(Direction::Next)
        .filter(|token| !is_kept(token))
        .any(|token| {
            (token.has_leading_comments() && Some(&token) != first.as_ref() && token != r_curly)
                || (token.has_trailing_comments() && Some(&token) != last.as_ref())
        })
}

/// Returns the condition for the `while` statement: the operand of `!test`, or
/// `test` negated with `!`.
fn loop_condition(test: AnyJsExpression) -> Option<AnyJsExpression> {
    let test = test.omit_parentheses();
    if let AnyJsExpression::JsUnaryExpression(unary) = &test
        && unary.operator().ok()? == JsUnaryOperator::LogicalNot
    {
        return unary.argument().ok()?.omit_parentheses().trim_trivia();
    }

    let test = test.trim_trivia()?;
    let operand = if test.precedence().ok()? < OperatorPrecedence::Unary
        || matches!(test, AnyJsExpression::JsArrowFunctionExpression(_))
    {
        make::parenthesized(test).into()
    } else {
        test
    };
    Some(make::js_unary_expression(make::token(T![!]), operand).into())
}
