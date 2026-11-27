use crate::{
    JsRuleAction,
    ast_utils::{keyword_directly_before, starts_unterminated_statement},
};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsArrowFunctionExpression, JsAwaitExpression,
    JsBinaryExpression, JsBinaryOperator, JsExportDefaultExpressionClause, JsExpressionStatement,
    JsInExpression, JsInstanceofExpression, JsLanguage, JsParenthesizedExpression, JsSyntaxKind,
    JsSyntaxKind::JS_REGEX_LITERAL, JsUnaryExpression, JsUnaryOperator, T, TsAsExpression,
    TsNonNullAssertionExpression, TsSatisfiesExpression, TsTypeAssertionExpression, is_negation,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{
    AstNode, AstNodeExt, BatchMutationExt, Direction, SyntaxKindSet, SyntaxTriviaPiece,
    TriviaPieceKind, WalkEvent,
};
use biome_rule_options::no_negated_comparison::NoNegatedComparisonOptions;

declare_lint_rule! {
    /// Disallow negating a whole equality comparison.
    ///
    /// Code such as `!(a === b)` makes the reader work out the comparison first and then
    /// reverse the result. The opposite operator gives the same result and reads more
    /// directly: `a !== b`.
    ///
    /// This rule checks the equality operators `===`, `!==`, `==`, and `!=`.
    /// Negated `<`, `>`, `<=`, and `>=` comparisons are allowed, because the opposite
    /// operator doesn't always give the same result. For example, `!(value > 0)` is `true`
    /// when `value` is `NaN` or `undefined`, but `value <= 0` is `false`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const isDifferent = !(a === b);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (!(typeof value === "undefined")) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const isDifferent = a !== b;
    /// if (typeof value !== "undefined") {}
    /// if (!(a === b && c === d)) {}
    /// if (!(a > b)) {}
    /// ```
    ///
    pub NoNegatedComparison {
        version: "next",
        name: "noNegatedComparison",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-negated-comparison").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoNegatedComparison {
    type Query = Ast<JsUnaryExpression>;
    /// The negated comparison, without the parentheses around it.
    type State = JsBinaryExpression;
    type Signals = Option<Self::State>;
    type Options = NoNegatedComparisonOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.operator().ok()? != JsUnaryOperator::LogicalNot {
            return None;
        }
        let AnyJsExpression::JsBinaryExpression(comparison) =
            node.argument().ok()?.omit_parentheses()
        else {
            return None;
        };
        opposite_operator(comparison.operator().ok()?)?;
        // In `!!(a === b)`, the outer `!` converts the result to a boolean on purpose.
        let parent = node
            .syntax()
            .ancestors()
            .skip(1)
            .find(|ancestor| !JsParenthesizedExpression::can_cast(ancestor.kind()));
        if parent.is_some_and(|parent| is_negation(&parent).is_some()) {
            return None;
        }
        Some(comparison)
    }

    fn diagnostic(ctx: &RuleContext<Self>, comparison: &Self::State) -> Option<RuleDiagnostic> {
        let operator = opposite_operator(comparison.operator().ok()?)?.to_string()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This comparison is negated as a whole."
                },
            )
            .note(markup! {
                "Reading it means working out the comparison and then reversing the result."
            })
            .note(markup! {
                "Use the opposite operator "<Emphasis>{operator}</Emphasis>" instead, which gives the same result."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, comparison: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let operator_kind = opposite_operator(comparison.operator().ok()?)?;
        let operator = comparison.operator_token().ok()?;
        let operator_text = operator_kind.to_string()?;
        let new_operator = make::token(operator_kind);

        let (kept, replacement) = if keeps_parentheses(node, comparison) {
            let argument = node.argument().ok()?;
            (
                argument.clone(),
                argument.replace_token(operator, new_operator)?,
            )
        } else {
            let comparison = AnyJsExpression::from(comparison.clone());
            (
                comparison.clone(),
                comparison.replace_token(operator, new_operator)?,
            )
        };
        let replacement = with_outside_trivia(node, &kept, replacement)?;

        let bang = node.operator_token().ok()?;
        let first_token = replacement.syntax().first_token()?;
        let old_node = AnyJsExpression::from(node.clone());
        // `foo` followed by `(a !== b)` on the next line would call `foo`.
        if ASI_HAZARD_KINDS.contains(first_token.kind()) && starts_unterminated_statement(&old_node)
        {
            return None;
        }
        // A line break right after `return`, `throw`, or `yield` ends the statement there, so
        // the comparison would no longer be its value.
        if bang
            .prev_token()
            .is_some_and(|token| LINE_BREAK_SENSITIVE_KEYWORDS.contains(token.kind()))
            && first_token
                .leading_trivia()
                .pieces()
                .skip(bang.leading_trivia().pieces().count())
                .any(|piece| is_line_break(&piece))
        {
            return None;
        }

        let mut mutation = ctx.root().begin();
        // In `return!(a === b)`, the keyword must not be joined to the `a` that follows it.
        if !first_token.kind().is_punct()
            && let Some(keyword) = keyword_directly_before(&old_node)
        {
            mutation.replace_token_discard_trivia(
                keyword.clone(),
                keyword.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            );
        }
        mutation.replace_node_discard_trivia(old_node, replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{operator_text}</Emphasis>" instead." },
            mutation,
        ))
    }
}

/// Returns the operator that gives the opposite result of `operator`, or `None` if `operator`
/// isn't an equality operator.
fn opposite_operator(operator: JsBinaryOperator) -> Option<JsSyntaxKind> {
    match operator {
        JsBinaryOperator::StrictEquality => Some(T![!==]),
        JsBinaryOperator::StrictInequality => Some(T![===]),
        JsBinaryOperator::Equality => Some(T![!=]),
        JsBinaryOperator::Inequality => Some(T![==]),
        _ => None,
    }
}

/// Returns `true` if the fixed comparison must stay in parentheses where `negation` is.
///
/// A comparison binds more loosely than `!`, so it needs parentheses as the operand of another
/// operator, such as in `a + !(b === c)`. It also needs them when it starts with `{`,
/// `function`, or `class` at the start of a statement, an arrow function body, or an
/// `export default` declaration, where those would start a block or a declaration instead.
/// Finally, it needs them when it contains an `in` operator inside the first part of a `for`
/// loop, such as in `for (let x = !(a === b in c); ;) {}`, where `in` would start a `for...in`
/// loop instead.
fn keeps_parentheses(negation: &JsUnaryExpression, comparison: &JsBinaryExpression) -> bool {
    if negation
        .syntax()
        .parent()
        .is_some_and(|parent| OPERAND_PARENT_KINDS.matches(parent.kind()))
    {
        return true;
    }
    if comparison
        .syntax()
        .preorder_of_kinds(JsInExpression::KIND_SET)
        .any(|event| matches!(event, WalkEvent::Enter(_)))
        && is_in_for_initializer(negation)
    {
        return true;
    }
    comparison
        .syntax()
        .first_token()
        .is_some_and(|token| STATEMENT_START_HAZARD_KINDS.contains(token.kind()))
        && starts_statement_like_position(negation)
}

/// Returns `true` if `negation` is inside the first part of a `for` or `for...in` loop, such as
/// `let x = !(a === b)` in `for (let x = !(a === b); ;) {}`.
fn is_in_for_initializer(negation: &JsUnaryExpression) -> bool {
    let start = negation.syntax().text_trimmed_range().start();
    match negation
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyJsStatement::cast)
    {
        Some(AnyJsStatement::JsForStatement(statement)) => statement
            .initializer()
            .is_some_and(|initializer| initializer.range().contains(start)),
        Some(AnyJsStatement::JsForInStatement(statement)) => statement
            .initializer()
            .is_ok_and(|initializer| initializer.range().contains(start)),
        _ => false,
    }
}

/// Returns `true` if `negation` is the first part of an expression statement, an arrow
/// function body, or an `export default` declaration.
fn starts_statement_like_position(negation: &JsUnaryExpression) -> bool {
    let first_token = negation.syntax().first_token();
    let mut child = negation.syntax().clone();
    for ancestor in negation.syntax().ancestors().skip(1) {
        if let Some(arrow) = JsArrowFunctionExpression::cast_ref(&ancestor) {
            return arrow.body().is_ok_and(|body| body.syntax() == &child);
        }
        // The only expression in `export default` is the exported one.
        if JsExportDefaultExpressionClause::can_cast(ancestor.kind()) {
            return true;
        }
        if ancestor.first_token() != first_token {
            return false;
        }
        if JsExpressionStatement::can_cast(ancestor.kind()) {
            return true;
        }
        child = ancestor;
    }
    false
}

/// Returns `replacement` with the comments and whitespace of the tokens in `negation` that are
/// outside `kept`, such as the `!` and the removed parentheses, moved before and after it.
fn with_outside_trivia(
    negation: &JsUnaryExpression,
    kept: &AnyJsExpression,
    replacement: AnyJsExpression,
) -> Option<AnyJsExpression> {
    let kept_range = kept.syntax().text_trimmed_range();
    let mut leading: Vec<SyntaxTriviaPiece<JsLanguage>> = Vec::new();
    let mut trailing: Vec<_> = replacement
        .syntax()
        .last_token()?
        .trailing_trivia()
        .pieces()
        .collect();
    for token in negation.syntax().descendants_tokens(Direction::Next) {
        let range = token.text_trimmed_range();
        let pieces = token
            .leading_trivia()
            .pieces()
            .chain(token.trailing_trivia().pieces());
        if range.end() <= kept_range.start() {
            leading.extend(pieces);
        } else if range.start() >= kept_range.end() {
            trailing.extend(pieces);
        }
    }
    leading.extend(
        replacement
            .syntax()
            .first_token()?
            .leading_trivia()
            .pieces(),
    );
    replacement
        .with_leading_trivia_pieces(leading)?
        .with_trailing_trivia_pieces(trailing)
}

/// Returns `true` if `piece` ends the current line.
fn is_line_break(piece: &SyntaxTriviaPiece<JsLanguage>) -> bool {
    piece.is_newline()
        || (piece.kind() == TriviaPieceKind::MultiLineComment
            && piece.text().contains(['\n', '\r', '\u{2028}', '\u{2029}']))
}

/// The parents in which `!(a === b)` is an operand that binds more tightly than a comparison.
const OPERAND_PARENT_KINDS: SyntaxKindSet<JsLanguage> = JsBinaryExpression::KIND_SET
    .union(JsInExpression::KIND_SET)
    .union(JsInstanceofExpression::KIND_SET)
    .union(JsUnaryExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(TsAsExpression::KIND_SET)
    .union(TsSatisfiesExpression::KIND_SET)
    .union(TsNonNullAssertionExpression::KIND_SET)
    .union(TsTypeAssertionExpression::KIND_SET);

/// The first tokens that change meaning at the start of a statement.
const STATEMENT_START_HAZARD_KINDS: TokenSet<JsSyntaxKind> =
    token_set![T!['{'], T![function], T![class], T![async], T![let]];

/// The first tokens that continue the previous statement when it has no semicolon.
const ASI_HAZARD_KINDS: TokenSet<JsSyntaxKind> = token_set![
    T!['('],
    T!['['],
    T!['`'],
    T![+],
    T![-],
    T![++],
    T![--],
    T![/],
    T![/=],
    T![<],
    JS_REGEX_LITERAL
];

/// The keywords after which a line break ends the statement.
const LINE_BREAK_SENSITIVE_KEYWORDS: TokenSet<JsSyntaxKind> =
    token_set![T![return], T![throw], T![yield]];
