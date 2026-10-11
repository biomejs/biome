use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::binary_like_expression::AnyJsBinaryLikeExpression;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsAssignmentExpression, JsAssignmentOperator,
    JsBinaryExpression, JsBinaryOperator, JsExpressionStatement, JsLogicalExpression, JsSyntaxKind,
    JsSyntaxToken, T,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxResult, declare_node_union};
use biome_rule_options::no_accidental_bitwise_operators::NoAccidentalBitwiseOperatorsOptions;

declare_lint_rule! {
    /// Disallow bitwise operators where a logical operator was likely intended.
    ///
    /// The bitwise operators `&`, `|`, and `|=` look a lot like the logical
    /// operators `&&`, `||`, and `||=`, so it is easy to type one when you meant
    /// the other. The two behave very differently:
    ///
    /// - Bitwise operators always evaluate both sides. In `obj & obj.prop`,
    ///   `obj.prop` is read even when `obj` is `null` or `undefined`, which
    ///   throws an error.
    /// - Bitwise operators turn both sides into whole numbers. In
    ///   `options | {}`, the object becomes `0`, so the result is a number
    ///   instead of `options` or `{}`.
    ///
    /// This rule only reports cases where a logical operator was clearly
    /// intended:
    ///
    /// - `&` between a variable and a property of that same variable, such as
    ///   `obj & obj.prop`.
    /// - `|` or `|=` with a right side that can never be a useful number: an
    ///   object, an array, a class, a function, a string, a template string, or
    ///   `true`/`false`.
    ///
    /// Bitwise math such as `flags & MASK` or `value | 0` is not reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (obj & obj.prop) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// options = options | {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// input |= "";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (obj && obj.prop) {}
    /// options = options || {};
    /// input ||= "";
    /// const masked = flags & MASK;
    /// const truncated = value | 0;
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow bitwise operators entirely, see [`noBitwiseOperators`](https://biomejs.dev/linter/rules/no-bitwise-operators/).
    ///
    pub NoAccidentalBitwiseOperators {
        version: "next",
        name: "noAccidentalBitwiseOperators",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-accidental-bitwise-operator").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyJsBitwiseOperation = JsBinaryExpression | JsAssignmentExpression
}

impl Rule for NoAccidentalBitwiseOperators {
    type Query = Ast<AnyJsBitwiseOperation>;
    type State = AccidentalOperator;
    type Signals = Option<Self::State>;
    type Options = NoAccidentalBitwiseOperatorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyJsBitwiseOperation::JsBinaryExpression(node) => match node.operator().ok()? {
                JsBinaryOperator::BitwiseAnd => {
                    is_short_circuit_guard(node).then_some(AccidentalOperator::And)
                }
                JsBinaryOperator::BitwiseOr => {
                    is_definitely_non_numeric(&node.right().ok()?).then_some(AccidentalOperator::Or)
                }
                _ => None,
            },
            AnyJsBitwiseOperation::JsAssignmentExpression(node) => (node.operator().ok()?
                == JsAssignmentOperator::BitwiseOrAssign
                && is_definitely_non_numeric(&node.right().ok()?))
            .then_some(AccidentalOperator::OrAssign),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, operator: &Self::State) -> Option<RuleDiagnostic> {
        let range = ctx.query().operator_token().ok()?.text_trimmed_range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The bitwise operator "<Emphasis>{operator.bitwise()}</Emphasis>" is used where the logical operator "<Emphasis>{operator.logical()}</Emphasis>" was likely intended."
                },
            )
            .note(markup! {
                "Bitwise operators always evaluate both sides and turn them into numbers, so the result is not the value of either side."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, operator: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        match ctx.query() {
            AnyJsBitwiseOperation::JsBinaryExpression(node) => {
                let logical = make::js_logical_expression(
                    node.left().ok()?,
                    replacement_token(&node.operator_token().ok()?, operator.logical_kind()),
                    node.right().ok()?,
                );
                let replacement = if needs_parentheses(node, &logical) {
                    if starts_unterminated_statement(node) {
                        return None;
                    }
                    // Keep the comments and whitespace around the expression
                    // outside of the added parentheses.
                    let leading = node.syntax().first_leading_trivia()?;
                    let trailing = node.syntax().last_trailing_trivia()?;
                    let logical = logical
                        .with_leading_trivia_pieces([])?
                        .with_trailing_trivia_pieces([])?;
                    make::parenthesized(logical)
                        .with_leading_trivia_pieces(leading.pieces())?
                        .with_trailing_trivia_pieces(trailing.pieces())?
                        .into()
                } else {
                    AnyJsExpression::from(logical)
                };
                mutation.replace_node(AnyJsExpression::from(node.clone()), replacement);
            }
            AnyJsBitwiseOperation::JsAssignmentExpression(node) => {
                let token = node.operator_token().ok()?;
                mutation.replace_token(
                    token.clone(),
                    replacement_token(&token, operator.logical_kind()),
                );
            }
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>{operator.bitwise()}</Emphasis>" with "<Emphasis>{operator.logical()}</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}

/// A bitwise operator that is likely a mistyped logical operator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccidentalOperator {
    /// `&`, likely meant as `&&`.
    And,
    /// `|`, likely meant as `||`.
    Or,
    /// `|=`, likely meant as `||=`.
    OrAssign,
}

impl AccidentalOperator {
    const fn bitwise(self) -> &'static str {
        match self {
            Self::And => "&",
            Self::Or => "|",
            Self::OrAssign => "|=",
        }
    }

    const fn logical(self) -> &'static str {
        match self {
            Self::And => "&&",
            Self::Or => "||",
            Self::OrAssign => "||=",
        }
    }

    const fn logical_kind(self) -> JsSyntaxKind {
        match self {
            Self::And => T![&&],
            Self::Or => T![||],
            Self::OrAssign => T![||=],
        }
    }
}

impl AnyJsBitwiseOperation {
    fn operator_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsBinaryExpression(node) => node.operator_token(),
            Self::JsAssignmentExpression(node) => node.operator_token(),
        }
    }
}

/// Returns `true` for `obj & obj.prop` and `obj & obj[key]`, where `&` was
/// likely meant to guard the property access with `&&`.
fn is_short_circuit_guard(node: &JsBinaryExpression) -> bool {
    let (Ok(left), Ok(right)) = (node.left(), node.right()) else {
        return false;
    };
    let AnyJsExpression::JsIdentifierExpression(left) = left.omit_parentheses() else {
        return false;
    };
    let object = match right.omit_parentheses() {
        AnyJsExpression::JsStaticMemberExpression(member) if !member.is_optional_chain() => {
            member.object()
        }
        AnyJsExpression::JsComputedMemberExpression(member) if !member.is_optional_chain() => {
            member.object()
        }
        _ => return false,
    };
    let Ok(AnyJsExpression::JsIdentifierExpression(object)) = object else {
        return false;
    };
    match (left.name(), object.name()) {
        (Ok(left), Ok(object)) => left.name().ok() == object.name().ok(),
        _ => false,
    }
}

/// Returns `true` for operands that can never be a meaningful number, such
/// as objects, strings, or functions. `null`, numbers, and variables are not
/// included, because they may be intended as numbers.
fn is_definitely_non_numeric(expression: &AnyJsExpression) -> bool {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::JsObjectExpression(_)
        | AnyJsExpression::JsArrayExpression(_)
        | AnyJsExpression::JsClassExpression(_)
        | AnyJsExpression::JsFunctionExpression(_)
        | AnyJsExpression::JsArrowFunctionExpression(_)
        | AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(_)
            | AnyJsLiteralExpression::JsBooleanLiteralExpression(_),
        ) => true,
        AnyJsExpression::JsTemplateExpression(template) => template.tag().is_none(),
        _ => false,
    }
}

/// Creates a token of `kind` that keeps the comments and whitespace around
/// `operator`.
fn replacement_token(operator: &JsSyntaxToken, kind: JsSyntaxKind) -> JsSyntaxToken {
    make::token(kind)
        .with_leading_trivia_pieces(operator.leading_trivia().pieces())
        .with_trailing_trivia_pieces(operator.trailing_trivia().pieces())
}

/// Returns `true` if `logical` must be wrapped in parentheses to keep the
/// meaning of the code when it replaces `node`.
///
/// Logical operators have a lower precedence than bitwise ones, so
/// `obj & obj.a | x` would become `obj && (obj.a | x)` without parentheses.
/// Mixing `??` with `&&` or `||` without parentheses is a syntax error.
fn needs_parentheses(node: &JsBinaryExpression, logical: &JsLogicalExpression) -> bool {
    let Some(parent) = node.syntax().parent() else {
        return false;
    };
    match AnyJsBinaryLikeExpression::cast(parent) {
        Some(AnyJsBinaryLikeExpression::JsLogicalExpression(parent)) => {
            parent.operator().ok() != logical.operator().ok()
        }
        Some(_) => true,
        None => false,
    }
}

/// Returns `true` if `node` starts a statement that follows a statement without
/// a semicolon. Wrapping `node` in parentheses would then turn the previous
/// line into a function call, as in `foo()\n(obj && obj.a) | x`.
fn starts_unterminated_statement(node: &JsBinaryExpression) -> bool {
    let Some(first_token) = node.syntax().first_token() else {
        return false;
    };
    let is_statement_start = node
        .syntax()
        .ancestors()
        .find_map(JsExpressionStatement::cast)
        .and_then(|statement| statement.syntax().first_token())
        .is_some_and(|token| token == first_token);
    is_statement_start
        && first_token
            .prev_token()
            .is_some_and(|previous| !STATEMENT_TERMINATORS.contains(previous.kind()))
}

const STATEMENT_TERMINATORS: TokenSet<JsSyntaxKind> = token_set![T![;], T!['{']];
