use crate::{JsRuleAction, services::semantic::Semantic};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxToken,
    JsUnaryExpression, JsUnaryOperator, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::no_typeof_undefined::NoTypeofUndefinedOptions;

declare_lint_rule! {
    /// Disallow comparing `undefined` using `typeof`.
    ///
    /// Checking whether a value is `undefined` with `typeof value === "undefined"`
    /// is more verbose than comparing the value directly with `undefined`.
    /// Direct comparison is easier to read when the value is known to be a local binding
    /// or a property access.
    ///
    /// The main exception is potentially missing global variables.
    /// `typeof missingGlobal === "undefined"` is runtime-safe even when `missingGlobal`
    /// was never declared, while `missingGlobal === undefined` can throw a `ReferenceError`.
    /// For that reason, this rule ignores unresolved or global identifiers by default.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let bar;
    ///
    /// typeof bar === "undefined";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let value;
    ///
    /// typeof value !== "undefined";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof foo.bar == "undefined";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// bar === undefined;
    /// ```
    ///
    /// ```js
    /// typeof value === "string";
    /// ```
    ///
    /// ```js
    /// typeof maybeGlobal === "undefined";
    /// ```
    ///
    /// ## Options
    ///
    /// The rule provides the option described below.
    ///
    /// ### checkGlobalVariables
    ///
    /// When `true`, the rule also reports comparisons against unresolved or global identifiers,
    /// such as `typeof missingGlobal === "undefined"`.
    ///
    /// This option exists because those checks are a special case: using `typeof` is runtime-safe
    /// for missing globals, while directly comparing the identifier with `undefined` can throw.
    /// Biome therefore ignores those cases by default unless you opt in.
    ///
    /// Default: `false`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "checkGlobalVariables": true
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// typeof missingGlobal === "string";
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// typeof missingGlobal === "undefined";
    /// ```
    pub NoTypeofUndefined {
        version: "next",
        name: "noTypeofUndefined",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-typeof-undefined").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Clone)]
pub struct RuleState {
    typeof_expr: JsUnaryExpression,
    undefined_literal: AnyJsExpression,
    operator: JsSyntaxToken,
    is_global_identifier: bool,
    should_suppress_fix: bool,
}

impl Rule for NoTypeofUndefined {
    type Query = Semantic<JsBinaryExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoTypeofUndefinedOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binary = ctx.query();
        let operator = binary.operator_token().ok()?;

        if !matches!(
            operator.kind(),
            JsSyntaxKind::EQ2 | JsSyntaxKind::EQ3 | JsSyntaxKind::NEQ | JsSyntaxKind::NEQ2
        ) {
            return None;
        }

        let typeof_expr = extract_typeof_expression(binary)?;
        let undefined_literal = extract_undefined_string(binary)?;
        let is_global_identifier = is_global_identifier_expression(&typeof_expr, ctx);
        if is_global_identifier && !ctx.options().check_global_variables() {
            return None;
        }

        Some(RuleState {
            typeof_expr,
            undefined_literal,
            operator,
            is_global_identifier,
            should_suppress_fix: is_global_identifier
                || requires_statement_boundary_preservation(binary),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let typeof_range = state.typeof_expr.range();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            typeof_range,
            markup! {
                "Compare with "<Emphasis>"undefined"</Emphasis>" directly instead of using "<Emphasis>"typeof"</Emphasis>"."
            },
        )
        .note(markup! {
            "Direct comparison is shorter and clearer when the value is a local binding or property access."
        });

        Some(if state.is_global_identifier {
            diagnostic.note(markup! {
                "This can be a potentially missing global. "<Emphasis>"typeof"</Emphasis>" is runtime-safe there, so review the change carefully before rewriting it."
            })
        } else {
            diagnostic.note(markup! {
                "Replace the comparison with "<Emphasis>"=== undefined"</Emphasis>" or "<Emphasis>"!== undefined"</Emphasis>"."
            })
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if state.should_suppress_fix {
            return None;
        }

        let argument = state.typeof_expr.argument().ok()?;
        let mut mutation = ctx.root().begin();

        mutation.replace_node(
            state.undefined_literal.clone(),
            make::js_identifier_expression(make::js_reference_identifier(make::ident("undefined")))
                .into(),
        );

        let next_operator = match state.operator.kind() {
            JsSyntaxKind::EQ2 => Some(T![===]),
            JsSyntaxKind::NEQ => Some(T![!==]),
            _ => None,
        };

        if let Some(next_operator) = next_operator {
            mutation.replace_token(state.operator.clone(), make::token(next_operator));
        }

        mutation.replace_node::<AnyJsExpression>(
            state.typeof_expr.clone().into(),
            replacement_argument(&state.typeof_expr, argument)?,
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Compare with "<Emphasis>"undefined"</Emphasis>" directly." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the `typeof` unary expression from the left-hand side of the binary expression.
///
/// For example, this extracts `typeof value` from:
///
/// ```js
/// typeof value === "undefined";
/// ```
fn extract_typeof_expression(binary: &JsBinaryExpression) -> Option<JsUnaryExpression> {
    let left = binary.left().ok()?.omit_parentheses();
    let unary = left.as_js_unary_expression()?;
    if unary.operator().ok()? == JsUnaryOperator::Typeof {
        Some(unary.clone())
    } else {
        None
    }
}

/// Returns the right-hand string literal expression when it is the exact string `"undefined"`.
///
/// For example, this extracts `"undefined"` from:
///
/// ```js
/// typeof value === "undefined";
/// ```
///
/// This intentionally excludes other static string forms like template literals.
fn extract_undefined_string(binary: &JsBinaryExpression) -> Option<AnyJsExpression> {
    let right = binary.right().ok()?.omit_parentheses();
    if matches!(right, AnyJsExpression::AnyJsLiteralExpression(_))
        && right.as_static_value()?.as_string_constant()? == "undefined"
    {
        Some(right)
    } else {
        None
    }
}

/// Checks whether the `typeof` operand is an unresolved or global identifier.
///
/// These identifiers are special because `typeof missingGlobal` is runtime-safe even when the
/// name does not exist, while directly reading `missingGlobal` can throw a `ReferenceError`.
///
/// For example, this returns `true` for:
///
/// ```js
/// typeof missingGlobal === "undefined";
/// ```
///
/// and `false` for:
///
/// ```js
/// typeof obj.value === "undefined";
/// ```
fn is_global_identifier_expression(
    typeof_expr: &JsUnaryExpression,
    ctx: &RuleContext<NoTypeofUndefined>,
) -> bool {
    let Some(reference) = typeof_expr.argument().ok().and_then(reference_identifier) else {
        return false;
    };

    ctx.model().binding(&reference).is_none()
}

/// Returns the operand reference when the expression is a plain identifier.
///
/// For example, this returns the reference for `value` in `typeof value`, but not for
/// `typeof obj.value`.
fn reference_identifier(expression: AnyJsExpression) -> Option<JsReferenceIdentifier> {
    expression.omit_parentheses().as_js_reference_identifier()
}

/// Detects statement-boundary cases where removing `typeof` can require extra syntax to preserve
/// parsing, such as `return` / `throw` line breaks or ASI-sensitive expressions that start with
/// `[` or `(`.
///
/// For example, this suppresses autofixes for cases like:
///
/// ```js
/// return typeof
///   value === "undefined";
///
/// foo
/// typeof [] === "undefined";
/// ```
fn requires_statement_boundary_preservation(binary: &JsBinaryExpression) -> bool {
    let Some(typeof_expr) = extract_typeof_expression(binary) else {
        return false;
    };

    let Some(argument_first_token) = typeof_expr
        .argument()
        .ok()
        .and_then(|arg| arg.syntax().first_token())
    else {
        return false;
    };

    if has_newline_before_token(&argument_first_token)
        && binary.syntax().parent().is_some_and(|parent| {
            matches!(
                parent.kind(),
                JsSyntaxKind::JS_RETURN_STATEMENT | JsSyntaxKind::JS_THROW_STATEMENT
            )
        })
    {
        return true;
    }

    binary
        .syntax()
        .parent()
        .is_some_and(|parent| parent.kind() == JsSyntaxKind::JS_EXPRESSION_STATEMENT)
        && matches!(
            argument_first_token.kind(),
            JsSyntaxKind::L_PAREN | JsSyntaxKind::L_BRACK
        )
}

/// Builds the replacement expression for the autofix after removing `typeof`.
///
/// This trims the leading whitespace that originally separated the operand from `typeof`, while
/// preserving comments that may sit between them. When a single-line comment appears after
/// `typeof`, the operand is wrapped in parentheses so the comment can stay attached to the opening
/// `(` without being dropped.
///
/// For example, this turns the operand in:
///
/// ```js
/// typeof value === "undefined";
/// ```
///
/// into `value`, without keeping the extra space that originally followed `typeof`.
fn replacement_argument(
    typeof_expr: &JsUnaryExpression,
    argument: AnyJsExpression,
) -> Option<AnyJsExpression> {
    let interstitial_trivia = interstitial_trivia(typeof_expr, &argument)?;

    if interstitial_trivia
        .iter()
        .any(|piece| piece.kind() == TriviaPieceKind::SingleLineComment)
    {
        let argument = argument.trim_leading_trivia()?;
        let l_paren = make::token(T!['(']).with_trailing_trivia_pieces(interstitial_trivia);
        let parenthesized = make::parenthesized(argument).with_l_paren_token(l_paren);
        Some(parenthesized.into())
    } else {
        argument.with_leading_trivia_pieces(interstitial_trivia)
    }
}

/// Collects the trivia that originally sat between `typeof` and its operand.
///
/// Leading whitespace is trimmed so the replacement expression does not inherit the padding that
/// was only needed after `typeof`.
fn interstitial_trivia(
    typeof_expr: &JsUnaryExpression,
    argument: &AnyJsExpression,
) -> Option<Vec<biome_rowan::SyntaxTriviaPiece<biome_js_syntax::JsLanguage>>> {
    let first_token = argument.syntax().first_token()?;
    let typeof_token = typeof_expr.operator_token().ok()?;
    let mut trimmed_leading = Vec::new();
    let mut trimming_leading_whitespace = true;

    for piece in typeof_token.trailing_trivia().pieces() {
        if trimming_leading_whitespace
            && matches!(
                piece.kind(),
                TriviaPieceKind::Whitespace | TriviaPieceKind::Newline
            )
        {
            continue;
        }

        trimming_leading_whitespace = false;
        trimmed_leading.push(piece);
    }

    for piece in first_token.leading_trivia().pieces() {
        if trimming_leading_whitespace
            && matches!(
                piece.kind(),
                TriviaPieceKind::Whitespace | TriviaPieceKind::Newline
            )
        {
            continue;
        }

        trimming_leading_whitespace = false;
        trimmed_leading.push(piece);
    }

    Some(trimmed_leading)
}

/// Returns `true` when the token has a leading newline in its trivia.
///
/// This is used to catch line-broken forms such as:
///
/// ```js
/// return typeof
///   value === "undefined";
/// ```
fn has_newline_before_token(token: &JsSyntaxToken) -> bool {
    token
        .leading_trivia()
        .pieces()
        .any(|piece| piece.is_newline())
}
