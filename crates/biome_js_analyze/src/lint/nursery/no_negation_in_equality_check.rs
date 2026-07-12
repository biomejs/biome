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
            )
            .note(markup! {
                    "Due to operator precedence, the negation binds more tightly than the equality operator. "
                    "The expression "<Emphasis>"!foo === bar"</Emphasis>" evaluates as "<Emphasis>"(!foo) === bar"</Emphasis>", not "<Emphasis>"!(foo === bar)"</Emphasis>"."
            })
            .note(markup! {
                "Replace "<Emphasis>"!left operator right"</Emphasis>" with the proper negated comparison, or wrap the comparison in parentheses with a negation."
            }),
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
        // (/, [, `, +, -), removing the `!` would change the parse.
        // `/` would become a regex literal, `[` property access,
        // `` ` `` a tagged template, and `+`/`-` a unary operator.
        // Skip the fix when any of these would be exposed at line start.
        {
            let has_preceding_newline = neg_op_token
                .leading_trivia()
                .pieces()
                .any(|p| p.kind() == TriviaPieceKind::Newline);
            if has_preceding_newline {
                let arg_text = negated_expr.syntax().text_trimmed().to_string();
                let first_char = arg_text.chars().next().unwrap_or('\0');
                if matches!(first_char, '/' | '[' | '`' | '+' | '-') {
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
        // Collect trivia from discarded tokens (the `!` operator and any
        // wrapping parentheses) so that comments are preserved in the fix.

        // Collect trivia pieces for the new left expression.
        let mut leading_for_left: Vec<SyntaxTriviaPiece<biome_js_syntax::JsLanguage>> = Vec::new();
        for p in neg_op_token.leading_trivia().pieces() {
            leading_for_left.push(p);
        }

        // Collect trivia pieces for the new operator.
        // Trailing trivia of `!` stays near the operator (Bug 3: cannot safely
        // move to left without losing comments in the tree replacement).
        let mut leading_for_op: Vec<SyntaxTriviaPiece<biome_js_syntax::JsLanguage>> = Vec::new();
        for p in neg_op_token.trailing_trivia().pieces() {
            leading_for_op.push(p);
        }
        let mut trailing_for_op: Vec<SyntaxTriviaPiece<biome_js_syntax::JsLanguage>> = Vec::new();

        // Walk through parenthesized wrappers from outer to inner.
        {
            let mut current = left.syntax().clone();
            loop {
                let Some(paren) = JsParenthesizedExpression::cast_ref(&current) else {
                    break;
                };
                if let Ok(lp) = paren.l_paren_token() {
                    for p in lp.leading_trivia().pieces() {
                        leading_for_left.push(p);
                    }
                    for p in lp.trailing_trivia().pieces() {
                        leading_for_left.push(p);
                    }
                }
                if let Ok(rp) = paren.r_paren_token() {
                    for p in rp.leading_trivia().pieces() {
                        leading_for_op.push(p);
                    }
                    for p in rp.trailing_trivia().pieces() {
                        leading_for_op.push(p);
                    }
                }
                match paren.expression() {
                    Ok(inner_expr) => current = inner_expr.into_syntax(),
                    Err(_) => break,
                }
            }
        }

        // Transfer the original comparison operator's trivia.
        for p in operator.leading_trivia().pieces() {
            leading_for_op.push(p);
        }
        for p in operator.trailing_trivia().pieces() {
            trailing_for_op.push(p);
        }

        // --- Build the new expression ---

        // 1. Build the new left expression with preserved trivia.
        // Use the node-level prepend_trivia_pieces which handles the token
        // replacement internally.
        let mut left_syntax = negated_expr.syntax().clone();
        if !leading_for_left.is_empty() {
            left_syntax = left_syntax.prepend_trivia_pieces(leading_for_left)?;
        }

        let mut new_left = AnyJsExpression::cast_ref(&left_syntax)?.clone();

        // Bug 1: Wrap function/class/object literals in parens to prevent
        // them from being reinterpreted as declarations/blocks at statement
        // level (e.g. `!function(){}===bar` removing `!` would expose
        // `function` at statement start, which is a function declaration).
        {
            let kind = new_left.syntax().kind();
            if matches!(kind, JS_FUNCTION_EXPRESSION | JS_CLASS_EXPRESSION | JS_OBJECT_EXPRESSION) {
                new_left = AnyJsExpression::from(make::parenthesized(new_left));
            }
        }

        // 2. Build the new operator token.
        // Use plain make::token (no automatic spacing) and copy trivia from
        // the old operator to avoid double whitespace (Bug 4 fix).
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
