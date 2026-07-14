use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsBinaryExpression, JsParenthesizedExpression, JsSyntaxKind::*, JsSyntaxNode};
use biome_js_syntax::is_negation;
use biome_rowan::{AstNode, BatchMutationExt, SyntaxTriviaPiece, TriviaPieceKind};
use biome_rule_options::no_negation_in_equality_check::NoNegationInEqualityCheckOptions;

declare_lint_rule! {
    /// Disallow negated expressions on the left side of an equality check.
    ///
    /// When a negation operator (`!`) is used on the left side of an equality check (`===` or `!==`),
    /// the negation binds more tightly than the comparison operator due to operator precedence.
    /// This means `!foo === bar` is evaluated as `(!foo) === bar`, which is almost always
    /// unintended. The developer likely meant `foo !== bar`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (!foo === bar) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (!foo !== bar) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (foo !== bar) {}
    /// ```
    ///
    /// ```js
    /// if (!(foo === bar)) {}
    /// ```
    ///
    pub NoNegationInEqualityCheck {
        version: "next",
        name: "noNegationInEqualityCheck",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-negation-in-equality-check").same()],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

/// Recursively walks through `JsParenthesizedExpression` wrappers to find
/// the innermost non-parenthesized syntax node.
fn skip_parens(mut node: JsSyntaxNode) -> JsSyntaxNode {
    while let Some(paren) = JsParenthesizedExpression::cast_ref(&node) {
        match paren.expression() {
            Ok(inner) => node = inner.syntax().clone(),
            Err(_) => break,
        }
    }
    node
}

impl Rule for NoNegationInEqualityCheck {
    type Query = Ast<JsBinaryExpression>;
    type State = JsBinaryExpression;
    type Signals = Option<Self::State>;
    type Options = NoNegationInEqualityCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let op = node.operator_token().ok()?;

        // Only check strict equality operators (=== and !==)
        if !matches!(op.kind(), EQ3 | NEQ2) {
            return None;
        }

        let left = node.left().ok()?;

        // Recursively unwrap parenthesized expressions, e.g. `((!foo)) === bar`
        let inner = skip_parens(left.syntax().clone());

        // Check if the unwrapped left side is a negation expression (!expr)
        let unary = is_negation(&inner)?;

        // Skip double negation (!!expr or !(...( !expr )...)) — this is
        // intentional boolean coercion, e.g. `!!foo === bar` or `!((!foo)) === bar`.
        let argument = unary.argument().ok()?;
        let arg_inner = skip_parens(argument.syntax().clone());
        if is_negation(&arg_inner).is_some() {
            return None;
        }

        Some(node.clone())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "A negation is used on the left side of this equality check."
                },
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = state;
        let left = node.left().ok()?;
        let right = node.right().ok()?;
        let operator = node.operator_token().ok()?;

        // Recursively unwrap parenthesized expressions to find the negation
        let inner = skip_parens(left.syntax().clone());
        let unary = is_negation(&inner)?;

        // The argument of `!` is the expression to use as the new left side
        let negated_expr = unary.argument().ok()?;
        let neg_op_token = unary.operator_token().ok()?;

        // --- ASI safety check ---
        // If a newline precedes the `!` operator and the argument starts with
        // a character that would continue the previous expression
        // (/, [, `, +, -, (, <), removing the `!` would change the parse.
        // `/` would become a regex literal, `[` property access,
        // `` ` `` a tagged template, `+`/`-` a unary operator, `(`
        // a function call, and `<` a JSX element or TS type assertion.
        // Skip the fix when any of these would be exposed at line start.
        //
        // MultiLineComment trivia also contains embedded newlines:
        // block comments are represented as MultiLineComment (not Newline)
        // pieces, but the line break inside them puts the argument at line
        // start just like a standalone newline would.
        //
        // Block comments may be attached as trailing trivia of the previous
        // token rather than leading trivia of `!` (e.g. `foo/*...\n...*/!x`).
        // We check both locations to avoid false-negative ASI hazards.
        {
            let has_preceding_newline = neg_op_token
                .leading_trivia()
                .pieces()
                .any(|p| {
                    p.kind() == TriviaPieceKind::Newline
                        || p.kind() == TriviaPieceKind::MultiLineComment
                });
            let prev_trailing_has_newline = neg_op_token
                .prev_token()
                .map(|t| {
                    t.trailing_trivia()
                        .pieces()
                        .any(|p| {
                            p.kind() == TriviaPieceKind::Newline
                                || p.kind() == TriviaPieceKind::MultiLineComment
                        })
                })
                .unwrap_or(false);
            if has_preceding_newline || prev_trailing_has_newline {
                let arg_text = negated_expr.syntax().text_trimmed().to_string();
                let first_char = arg_text.chars().next().unwrap_or('\0');
                if matches!(first_char, '/' | '[' | '`' | '+' | '-' | '(' | '<') {
                    return None;
                }
            }
        }

        // Flip the operator: === → !==, !== → ===
        let new_op_kind = match operator.kind() {
            EQ3 => NEQ2,
            NEQ2 => EQ3,
            _ => return None,
        };

        // --- Trivia transfer ---
        // Collect trivia from discarded tokens so that comments are
        // preserved in the fix output.
        //
        // Discarded tokens:
        //   - Outer `(` and `)` from JsParenthesizedExpression wrappers
        //   - The `!` operator token from the JsUnaryExpression
        //   - The original comparison operator (`===` or `!==`)
        //
        // Note on replace_node behavior:
        //   `BatchMutation::replace_node` auto-copies the OLD node's first
        //   token's leading trivia to the NEW node's first token's leading
        //   trivia (it *replaces*, not chains). The new node's first token
        //   is the surviving argument expression (e.g., `foo`). The new
        //   node's middle token is the new comparison operator, which is
        //   untouched by replace_node.
        //
        // Strategy: put all manually-collected trivia into the new
        // operator's trivia vectors (leading_for_op / trailing_for_op).
        // The new operator is a middle token — safe from replace_node's
        // first/last-token replacement. The old first token's leading
        // trivia is handled automatically by replace_node.

        let mut leading_for_op: Vec<SyntaxTriviaPiece<biome_js_syntax::JsLanguage>> = Vec::new();
        let mut trailing_for_op: Vec<SyntaxTriviaPiece<biome_js_syntax::JsLanguage>> = Vec::new();

        // Phase 1: Collect trivia from parenthesized wrappers.
        // Walk from outer to inner, collecting COMMENT trivia from
        // discarded `(` and `)` tokens and routing it to the new operator.
        // We skip whitespace pieces to avoid double-spacing with the
        // operator's own trivia.
        //
        // IMPORTANT: We skip l_paren.leading_trivia() because for the
        // outermost paren, this IS the old binary's first token leading
        // trivia, which replace_node auto-copies. Collecting it here
        // would put it on the new operator too, causing duplication and
        // unwanted line breaks.
        //
        // We DO collect COMMENT pieces from l_paren.trailing_trivia()
        // (gap between `(` and the inner expression, e.g. `/* keep */`).
        // We also collect COMMENT pieces from r_paren trivia.
        {
            let mut current = left.syntax().clone();
            let mut is_outermost = true;
            while let Some(paren) = JsParenthesizedExpression::cast_ref(&current) {
                if let Ok(lp) = paren.l_paren_token() {
                    // Skip leading trivia on outermost `(` — replace_node
                    // handles it. Collect for inner parens (nested case).
                    if !is_outermost {
                        for p in lp.leading_trivia().pieces() {
                            leading_for_op.push(p);
                        }
                    }
                    // Only collect COMMENT pieces from trailing trivia.
                    // Whitespace pieces are skipped to avoid double-spacing.
                    for p in lp.trailing_trivia().pieces().filter(|p| p.kind().is_comment()) {
                        leading_for_op.push(p);
                    }
                }
                if let Ok(rp) = paren.r_paren_token() {
                    // Only collect COMMENT pieces from leading trivia —
                    // whitespace would duplicate with l_paren padding.
                    for p in rp.leading_trivia().pieces().filter(|p| p.kind().is_comment()) {
                        leading_for_op.push(p);
                    }
                    // Collect ALL trailing trivia pieces. This is the
                    // gap between `)` and the next token (operator or
                    // outer `)`) — we need the spacing here.
                    for p in rp.trailing_trivia().pieces() {
                        leading_for_op.push(p);
                    }
                }
                match paren.expression() {
                    Ok(inner_expr) => current = inner_expr.into_syntax(),
                    Err(_) => break,
                }
                is_outermost = false;
            }
        }

        // Phase 2: Collect trivia from the removed `!` operator.
        //
        // We intentionally do NOT collect neg_op_token.leading_trivia():
        // for non-parenthesized cases, neg_op_token IS the old binary's
        // first token, and its leading trivia is auto-copied by
        // replace_node. Collecting it here would cause duplication.
        //
        // For parenthesized cases, the trivia between `(` and `!` may be
        // stored as trailing trivia of `(` already, but it may also be
        // stored as leading trivia of `!`. To avoid double-counting, we
        // skip neg_op_token.leading_trivia() entirely and rely on the
        // parenthesized wrapper's `l_paren.trailing_trivia()` collected in
        // Phase 1 (which covers the same gap from the left side).
        //
        // We DO collect neg_op_token.trailing_trivia(): this is the trivia
        // between `!` and its argument (e.g., `/* keep */` in
        // `!/* keep */foo`). This trivia is not covered by replace_node
        // and not covered by the paren wrapper loop.
        for p in neg_op_token.trailing_trivia().pieces() {
            leading_for_op.push(p);
        }

        // Phase 3: Transfer the original comparison operator's trivia.
        for p in operator.leading_trivia().pieces() {
            leading_for_op.push(p);
        }
        for p in operator.trailing_trivia().pieces() {
            trailing_for_op.push(p);
        }

        // --- Build the new expression ---

        // 1. The new left expression: use the argument expression directly.
        //    replace_node will auto-copy the old first token's leading trivia
        //    to it, so no manual prepend is needed.
        let mut new_left = negated_expr;

        // Wrap function/class/object literals in parens to prevent them
        // from being reinterpreted as declarations/blocks at statement
        // level (e.g. `!function(){}===bar` removing `!` would expose
        // `function` at statement start, which is a function declaration).
        {
            let kind = new_left.syntax().kind();
            if matches!(kind, JS_FUNCTION_EXPRESSION | JS_CLASS_EXPRESSION | JS_OBJECT_EXPRESSION) {
                new_left = AnyJsExpression::from(make::parenthesized(new_left));
            }
        }

        // 2. Build the new operator token with preserved trivia.
        let mut new_op = make::token(new_op_kind);
        if !leading_for_op.is_empty() {
            new_op = new_op.prepend_trivia_pieces(leading_for_op);
        }
        if !trailing_for_op.is_empty() {
            new_op = new_op.append_trivia_pieces(trailing_for_op);
        }

        let new_binary = make::js_binary_expression(
            new_left,
            new_op,
            right,
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::JsBinaryExpression(node.clone()),
            AnyJsExpression::from(new_binary),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the proper negated comparison operator instead." }.to_owned(),
            mutation,
        ))
    }
}
