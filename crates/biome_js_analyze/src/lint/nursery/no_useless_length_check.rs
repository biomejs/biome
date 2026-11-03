use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsForInOrOfInitializer, AnyJsLiteralExpression, AnyJsStatement,
    JsBinaryExpression, JsBinaryOperator, JsForOfStatement, JsIdentifierBinding, JsIfStatement,
    JsLanguage, JsLogicalExpression, JsLogicalOperator, JsReferenceIdentifier, JsSyntaxKind, T,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{
    AstNode, AstNodeList, BatchMutationExt, Direction, SyntaxKindSet, TextRange, WalkEvent,
};
use biome_rule_options::no_useless_length_check::NoUselessLengthCheckOptions;

use crate::{
    JsRuleAction, ast_utils::starts_unterminated_statement,
    utils::same_reference::is_same_reference,
};

declare_lint_rule! {
    /// Disallow checking the length of an array when the result doesn't depend on it.
    ///
    /// Some code checks whether an array is empty even though the code next to it already
    /// handles empty arrays:
    ///
    /// - `some()` returns `false` for an empty array, so `array.length > 0 && array.some(callback)`
    ///   gives the same result as `array.some(callback)`.
    /// - `every()` returns `true` for an empty array, so `array.length === 0 || array.every(callback)`
    ///   gives the same result as `array.every(callback)`.
    /// - A `for...of` loop doesn't run its body for an empty array, so an `if` statement that only
    ///   checks that the array isn't empty before the loop is unnecessary.
    ///
    /// Removing the check makes the code shorter and easier to read.
    ///
    /// The rule only reports checks written as `.length === 0`, `.length !== 0`, or `.length > 0`.
    /// It reports a check next to `some()` or `every()` only when both are joined by the same
    /// `&&` or `||` operator.
    ///
    /// The rule assumes that the value is an array. The fix is unsafe because the value might be
    /// another kind of object with its own `length`, `some()`, or `every()`. It's also unsafe
    /// because removing a check written before the call runs the call's arguments even when the
    /// array is empty, and removing a check written after the call ignores a callback that changes
    /// the array.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (array.length === 0 || array.every(Boolean));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (array.length !== 0 && array.some(Boolean));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (array.length > 0 && array.some(Boolean));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const isAllTruthyOrEmpty = array.length === 0 || array.every(Boolean);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (array.length > 0) {
    ///     for (const element of array) {
    ///         use(element);
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (array.every(Boolean));
    /// if (array.some(Boolean));
    /// for (const element of array) {
    ///     use(element);
    /// }
    /// if (array.length === 0 || anotherCheck() || array.every(Boolean));
    /// const isNonEmptyAllTruthyArray = array.length > 0 && array.every(Boolean);
    /// const isEmptyArrayOrAllTruthy = array.length === 0 || array.some(Boolean);
    /// ```
    ///
    /// ## See Also
    ///
    /// - To choose how length checks are written, see [`useExplicitLengthCheck`](https://biomejs.dev/linter/rules/use-explicit-length-check/).
    ///
    pub NoUselessLengthCheck {
        version: "next",
        name: "noUselessLengthCheck",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-useless-length-check").same()],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessLengthCheck {
    type Query = Ast<JsBinaryExpression>;
    type State = UselessLengthCheck;
    type Signals = Option<Self::State>;
    type Options = NoUselessLengthCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let check = ctx.query();
        let (array, kind) = length_check(check)?;
        let outer = outermost_parentheses(AnyJsExpression::from(check.clone()));
        let parent = outer.syntax().parent()?;

        if let Some(logical) = JsLogicalExpression::cast_ref(&parent) {
            let operator = logical.operator().ok()?;
            let method = match (kind, operator) {
                (LengthCheck::Empty, JsLogicalOperator::LogicalOr) => ArrayMethod::Every,
                (LengthCheck::NonEmpty, JsLogicalOperator::LogicalAnd) => ArrayMethod::Some,
                _ => return None,
            };
            let is_beside_call = [Side::Right, Side::Left].into_iter().any(|side| {
                adjacent_operand(&outer, operator, side)
                    .is_some_and(|operand| is_matching_call(&operand, method, &array))
            });
            return is_beside_call.then_some(UselessLengthCheck::BesideCall {
                method,
                logical: logical.clone(),
            });
        }

        let if_statement = JsIfStatement::cast(parent)?;
        if kind != LengthCheck::NonEmpty || if_statement.else_clause().is_some() {
            return None;
        }
        let for_of = guarded_for_of(&if_statement)?;
        if for_of.await_token().is_some()
            || !is_same_reference(array, for_of.expression().ok()?)?
            || has_loop_variable_in_expression(&for_of)?
        {
            return None;
        }
        Some(UselessLengthCheck::ForOfGuard {
            if_statement,
            for_of,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            UselessLengthCheck::BesideCall { method, .. } => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                match method {
                    ArrayMethod::Every => markup! {
                        "This check for an empty array is unnecessary."
                    },
                    ArrayMethod::Some => markup! {
                        "This check for a non-empty array is unnecessary."
                    },
                },
            )
            .note(match method {
                ArrayMethod::Every => markup! {
                    <Emphasis>"every()"</Emphasis>" returns "<Emphasis>"true"</Emphasis>" for an empty array, so the result is the same without the check."
                },
                ArrayMethod::Some => markup! {
                    <Emphasis>"some()"</Emphasis>" returns "<Emphasis>"false"</Emphasis>" for an empty array, so the result is the same without the check."
                },
            }),
            UselessLengthCheck::ForOfGuard { .. } => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This check for a non-empty array is unnecessary."
                },
            )
            .note(markup! {
                "A "<Emphasis>"for...of"</Emphasis>" loop doesn't run its body for an empty array, so the code behaves the same without the check."
            }),
        };
        Some(diagnostic.note(markup! {
            "Remove the length check."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        match state {
            UselessLengthCheck::BesideCall { logical, .. } => {
                let left = logical.left().ok()?;
                let right = logical.right().ok()?;
                let left_range = left.syntax().text_trimmed_range();
                let right_range = right.syntax().text_trimmed_range();
                let is_leading =
                    left_range.contains_range(ctx.query().syntax().text_trimmed_range());
                // The removed text holds the check and the operator.
                let (kept, removed) = if is_leading {
                    (
                        right,
                        TextRange::new(left_range.start(), right_range.start()),
                    )
                } else {
                    (left, TextRange::new(left_range.end(), right_range.end()))
                };
                if has_comment_in_ranges(logical, &[removed]) {
                    return None;
                }
                let logical = AnyJsExpression::from(logical.clone());
                // In `foo()\narray.length > 0 && (array).some(f)`, the remaining `(` would call
                // the result of `foo()`.
                if is_leading
                    && starts_unterminated_statement(&logical)
                    && STATEMENT_CONTINUATION_TOKENS.contains(kept.syntax().first_token()?.kind())
                {
                    return None;
                }
                mutation.replace_node(logical, kept);
            }
            UselessLengthCheck::ForOfGuard {
                if_statement,
                for_of,
            } => {
                let if_range = if_statement.syntax().text_trimmed_range();
                let loop_range = for_of.syntax().text_trimmed_range();
                let removed = [
                    TextRange::new(if_range.start(), loop_range.start()),
                    TextRange::new(loop_range.end(), if_range.end()),
                ];
                if has_comment_in_ranges(if_statement, &removed) {
                    return None;
                }
                mutation.replace_node(
                    AnyJsStatement::from(if_statement.clone()),
                    AnyJsStatement::from(for_of.clone()),
                );
            }
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the length check." }.to_owned(),
            mutation,
        ))
    }
}

pub enum UselessLengthCheck {
    /// The check and a call to `every()` or `some()` on the same array are next to each other
    /// in `logical`, or in a chain of logical expressions with the same operator that contains
    /// `logical`. `logical` is the expression whose left or right operand is the check.
    BesideCall {
        method: ArrayMethod,
        logical: JsLogicalExpression,
    },
    /// The check is the condition of `if_statement`, whose only statement is `for_of`.
    ForOfGuard {
        if_statement: JsIfStatement,
        for_of: JsForOfStatement,
    },
}

#[derive(Clone, Copy)]
pub enum ArrayMethod {
    Every,
    Some,
}

impl ArrayMethod {
    const fn name(self) -> &'static str {
        match self {
            Self::Every => "every",
            Self::Some => "some",
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum LengthCheck {
    /// `.length === 0`
    Empty,
    /// `.length !== 0` or `.length > 0`
    NonEmpty,
}

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

/// Tokens that continue the previous line's expression when a line starts with them.
const STATEMENT_CONTINUATION_TOKENS: TokenSet<JsSyntaxKind> =
    token_set![T!['('], T!['['], T!['`'], T![<], T![+], T![-], T![/]];

const LOOP_VARIABLE_KINDS: SyntaxKindSet<JsLanguage> = JsIdentifierBinding::KIND_SET;

const REFERENCE_KINDS: SyntaxKindSet<JsLanguage> = JsReferenceIdentifier::KIND_SET;

/// Returns the array whose length `check` compares with `0`, and what the comparison checks.
///
/// Only `.length === 0`, `.length !== 0`, and `.length > 0` are recognized. Returns `None` for
/// any other comparison, and when the `length` property is read with optional chaining (`?.`).
fn length_check(check: &JsBinaryExpression) -> Option<(AnyJsExpression, LengthCheck)> {
    let kind = match check.operator().ok()? {
        JsBinaryOperator::StrictEquality => LengthCheck::Empty,
        JsBinaryOperator::StrictInequality | JsBinaryOperator::GreaterThan => LengthCheck::NonEmpty,
        _ => return None,
    };
    let AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNumberLiteralExpression(
        zero,
    )) = check.right().ok()?.omit_parentheses()
    else {
        return None;
    };
    if zero.value_token().ok()?.text_trimmed() != "0" {
        return None;
    }
    let AnyJsExpression::JsStaticMemberExpression(member) = check.left().ok()?.omit_parentheses()
    else {
        return None;
    };
    if member.is_optional_chain()
        || member
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .text_trimmed()
            != "length"
    {
        return None;
    }
    Some((member.object().ok()?, kind))
}

/// Returns the outermost parenthesized expression that wraps `expression`, or `expression`
/// itself if it isn't wrapped in parentheses.
fn outermost_parentheses(expression: AnyJsExpression) -> AnyJsExpression {
    expression
        .syntax()
        .ancestors()
        .skip(1)
        .map_while(AnyJsExpression::cast)
        .take_while(|ancestor| matches!(ancestor, AnyJsExpression::JsParenthesizedExpression(_)))
        .last()
        .unwrap_or(expression)
}

/// Returns the operand that comes right before or after `operand` (depending on `side`) when a
/// chain of logical expressions with `operator` is read as a flat list.
///
/// For example, in `a && (b && c)`, the operand after `a` is `b`, and the operand before `c` is
/// `b`. Parentheses around operands are ignored. A logical expression with a different operator
/// counts as a single operand, so in `a && (b || c)`, the operand after `a` is `b || c`.
fn adjacent_operand(
    operand: &AnyJsExpression,
    operator: JsLogicalOperator,
    side: Side,
) -> Option<AnyJsExpression> {
    let mut current = operand.clone();
    loop {
        let parent = JsLogicalExpression::cast(current.syntax().parent()?)?;
        if parent.operator().ok()? != operator {
            return None;
        }
        let (near, far) = match side {
            Side::Left => (parent.right().ok()?, parent.left().ok()?),
            Side::Right => (parent.left().ok()?, parent.right().ok()?),
        };
        if near.syntax() == current.syntax() {
            return edge_operand(far, operator, side);
        }
        current = outermost_parentheses(AnyJsExpression::from(parent));
    }
}

/// Returns the operand at the edge of `expression` that faces the opposite `side`, when
/// `expression` is read as a flat chain of logical expressions with `operator`.
///
/// For example, for `(a && b) && c`, the operand that faces the right side is `a`.
fn edge_operand(
    mut expression: AnyJsExpression,
    operator: JsLogicalOperator,
    side: Side,
) -> Option<AnyJsExpression> {
    loop {
        expression = expression.omit_parentheses();
        let AnyJsExpression::JsLogicalExpression(logical) = &expression else {
            return Some(expression);
        };
        if logical.operator().ok()? != operator {
            return Some(expression);
        }
        expression = match side {
            Side::Left => logical.right().ok()?,
            Side::Right => logical.left().ok()?,
        };
    }
}

/// Returns whether `expression` calls `method` on `array`, as in `array.some(callback)`.
fn is_matching_call(
    expression: &AnyJsExpression,
    method: ArrayMethod,
    array: &AnyJsExpression,
) -> bool {
    let AnyJsExpression::JsCallExpression(call) = expression else {
        return false;
    };
    if expression.is_optional_chain() {
        return false;
    }
    let Some(AnyJsExpression::JsStaticMemberExpression(callee)) =
        call.callee().ok().map(AnyJsExpression::omit_parentheses)
    else {
        return false;
    };
    !callee.is_optional_chain()
        && callee
            .member()
            .ok()
            .and_then(|member| member.as_js_name()?.value_token().ok())
            .is_some_and(|name| name.text_trimmed() == method.name())
        && callee
            .object()
            .ok()
            .and_then(|object| is_same_reference(object, array.clone()))
            .unwrap_or(false)
}

/// Returns the `for...of` loop that is the only statement of `if_statement`.
fn guarded_for_of(if_statement: &JsIfStatement) -> Option<JsForOfStatement> {
    let statement = match if_statement.consequent().ok()? {
        AnyJsStatement::JsBlockStatement(block) => {
            let statements = block.statements();
            if statements.len() != 1 {
                return None;
            }
            statements.first()?
        }
        statement => statement,
    };
    JsForOfStatement::cast(statement.into_syntax())
}

/// Returns whether the iterated expression of `for_of` reads a variable that the loop declares
/// with `let`, `const`, or `using`.
///
/// Such a variable isn't available yet when the iterated expression runs, so in
/// `for (const array of array)`, the iterated `array` is a different variable from the `array`
/// whose length is checked.
///
/// Returns `None` if the syntax tree is incomplete.
fn has_loop_variable_in_expression(for_of: &JsForOfStatement) -> Option<bool> {
    let AnyJsForInOrOfInitializer::JsForVariableDeclaration(declaration) =
        for_of.initializer().ok()?
    else {
        return Some(false);
    };
    // `var` declares the variable for the whole function, outside the loop.
    if declaration.kind_token().ok()?.kind() == T![var] {
        return Some(false);
    }
    let expression = for_of.expression().ok()?;
    Some(
        declaration
            .syntax()
            .preorder_of_kinds(LOOP_VARIABLE_KINDS)
            .filter_map(|event| match event {
                WalkEvent::Enter(node) => JsIdentifierBinding::cast(node)?.name_token().ok(),
                WalkEvent::Leave(_) => None,
            })
            .any(|binding| {
                expression
                    .syntax()
                    .preorder_of_kinds(REFERENCE_KINDS)
                    .filter_map(|event| match event {
                        WalkEvent::Enter(node) => {
                            JsReferenceIdentifier::cast(node)?.value_token().ok()
                        }
                        WalkEvent::Leave(_) => None,
                    })
                    .any(|reference| reference.text_trimmed() == binding.text_trimmed())
            }),
    )
}

/// Returns whether a comment inside `node` overlaps any of `ranges`.
fn has_comment_in_ranges(node: &impl AstNode<Language = JsLanguage>, ranges: &[TextRange]) -> bool {
    node.syntax()
        .descendants_tokens(Direction::Next)
        .any(|token| {
            token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
                .filter(|piece| piece.is_comments())
                .any(|piece| {
                    ranges.iter().any(|range| {
                        range
                            .intersect(piece.text_range())
                            .is_some_and(|overlap| !overlap.is_empty())
                    })
                })
        })
}
