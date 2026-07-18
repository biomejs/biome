use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsBinaryExpression, JsParenthesizedExpression, JsSyntaxKind::*};
use biome_js_syntax::is_negation;
use biome_parser::token_set;
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

impl Rule for NoNegationInEqualityCheck {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoNegationInEqualityCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let op = node.operator_token().ok()?;

        // Only check strict equality operators (=== and !==)
        if !token_set![EQ3, NEQ2].contains(op.kind()) {
            return None;
        }

        let left = node.left().ok()?;

        // Recursively unwrap parenthesized expressions, e.g. `((!foo)) === bar`
        let inner = left.omit_parentheses();

        // Check if the unwrapped left side is a negation expression (!expr)
        let unary = is_negation(inner.syntax())?;

        // Skip double negation (!!expr or !(...( !expr )...)) — this is
        // intentional boolean coercion, e.g. `!!foo === bar` or `!((!foo)) === bar`.
        let argument = unary.argument().ok()?;
        let arg_inner = argument.omit_parentheses();
        if is_negation(arg_inner.syntax()).is_some() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "A negation is used on the left side of this equality check."
                },
            )
            .note(markup! {
                "Due to operator precedence, the negation binds more tightly than the equality operator. "
                "The expression "<Emphasis>"!foo === bar"</Emphasis>" evaluates as "<Emphasis>"(!foo) === bar"</Emphasis>", not "<Emphasis>"!(foo === bar)"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let left = node.left().ok()?;
        let right = node.right().ok()?;
        let operator = node.operator_token().ok()?;

        // Recursively unwrap parenthesized expressions to find the negation
        let inner = left.omit_parentheses();
        let unary = is_negation(inner.syntax())?;

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
        //
        // The `has_newline` flag is also used below after wrapping
        // function/class/object expressions in parens — the generated `(`
        // is itself an ASI hazard when preceded by a newline.
        let has_preceding_newline = neg_op_token
            .leading_trivia()
            .pieces()
            .any(|p| {
                p.kind() == TriviaPieceKind::Newline
                    || p.kind() == TriviaPieceKind::MultiLineComment
            });
        let prev_trailing_has_newline = neg_op_token
            .prev_token()
            .is_some_and(|t| {
                t.trailing_trivia()
                    .pieces()
                    .any(|p| {
                        p.kind() == TriviaPieceKind::Newline
                            || p.kind() == TriviaPieceKind::MultiLineComment
                    })
            });
        let has_newline = has_preceding_newline || prev_trailing_has_newline;
        if has_newline {
            let arg_text = negated_expr.syntax().text_trimmed().to_string();
            let first_char = arg_text.chars().next().unwrap_or('\0');
            if matches!(first_char, '/' | '[' | '`' | '+' | '-' | '(' | '<') {
                return None;
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

        // Phase 1: Collect trivia in document (source) order.
        //
        // The surviving expression is the old unary argument (e.g. `foo`).
        // Everything before it (`(`, `!`) and after it (`)`, `===`) becomes
        // trivia routed to the new comparison operator.
        //
        // Document order of discarded tokens:
        //   outer `(` → … → inner `(` → `!` → inner `)` → … → outer `)` → `===`
        //
        // We walk paren wrappers outer→inner collecting `(` trivia, then
        // collect `!` trivia, then walk back inner→outer collecting `)`
        // trivia, then collect the original operator trivia. This preserves
        // the original left-to-right order of comments and whitespace.
        //
        // Whitespace from `l_paren.trailing` and `r_paren.leading` is
        // filtered to COMMENTS + NEWLINES only — spaces/tabs were padding
        // around deleted tokens and would cause unwanted extra spacing.
        // Newlines are preserved because they are needed to terminate line
        // comments (without a trailing newline a `//` comment would
        // comment out the replacement operator).
        // Whitespace from `r_paren.trailing` and the `!` operator is
        // preserved in full because it provides the proper gap between
        // the surviving expression and the new operator.

        // Accumulate paren nodes so we can traverse back inner→outer.
        let mut paren_nodes: Vec<JsParenthesizedExpression> = Vec::new();

        // Step 1a: Walk outer→inner, collecting `(` trivia.
        {
            let mut current = node.left().ok()?.into_syntax();
            let mut is_outermost = true;
            while let Some(paren) = JsParenthesizedExpression::cast_ref(&current) {
                if let Ok(lp) = paren.l_paren_token() {
                    // Skip leading trivia on outermost `(` — replace_node
                    // auto-copies it. Collect for inner parens (nested).
                    if !is_outermost {
                        for p in lp.leading_trivia().pieces() {
                            leading_for_op.push(p);
                        }
                    }
                    // COMMENTS + NEWLINES only — spaces/tabs were
                    // padding around the deleted `(`, not needed.
                    for p in lp.trailing_trivia().pieces().filter(|p| p.kind().is_comment() || p.kind() == TriviaPieceKind::Newline) {
                        leading_for_op.push(p);
                    }
                }
                paren_nodes.push(paren.clone());
                match paren.expression() {
                    Ok(inner_expr) => current = inner_expr.into_syntax(),
                    Err(_) => break,
                }
                is_outermost = false;
            }
        }

        // Step 1b: Collect `!` operator trivia.
        //
        // For non-parenthesized cases (!foo === bar), neg_op_token IS the
        // binary's first token and its leading trivia is auto-copied by
        // replace_node — skip to avoid duplication.
        //
        // For parenthesized cases ((!foo) === bar), the outermost `(` is
        // the binary's first token, NOT `!`. replace_node copies `(`'s
        // leading trivia but leaves `!`'s leading trivia untouched.
        // Comments between `(` and `!` (e.g., `(/* keep */ !foo)`) may be
        // stored as leading trivia of `!` rather than trailing trivia of
        // `(`, depending on parser internals. To avoid losing them, we
        // collect neg_op_token.leading_trivia() when paren wrappers exist.
        //
        // We always collect neg_op_token.trailing_trivia(): trivia between
        // `!` and its argument (e.g., `/* keep */` in `!/* keep */foo`).
        let has_paren_wrappers = !paren_nodes.is_empty();
        if has_paren_wrappers {
            for p in neg_op_token.leading_trivia().pieces() {
                leading_for_op.push(p);
            }
        }
        for p in neg_op_token.trailing_trivia().pieces() {
            leading_for_op.push(p);
        }

        // Collect the argument's own leading trivia so that comments
        // attached directly to the surviving expression (e.g. `!/*x*/foo`
        // where the parser stores `/*x*/` as `foo`'s leading trivia rather
        // than `!`'s trailing trivia) are not clobbered by replace_node.
        if let Some(arg_first) = negated_expr.syntax().first_token() {
            for p in arg_first.leading_trivia().pieces() {
                leading_for_op.push(p);
            }
        }

        // Step 1c: Walk inner→outer, collecting `)` trivia.
        for paren in paren_nodes.iter().rev() {
            if let Ok(rp) = paren.r_paren_token() {
                // COMMENTS + NEWLINES only — spaces/tabs were padding
                // around the deleted `)`, not needed.
                for p in rp.leading_trivia().pieces().filter(|p| p.kind().is_comment() || p.kind() == TriviaPieceKind::Newline) {
                    leading_for_op.push(p);
                }
                // Collect ALL trailing trivia pieces — this is the gap
                // between `)` and the next token (`===` or outer `)`).
                for p in rp.trailing_trivia().pieces() {
                    leading_for_op.push(p);
                }
            }
        }

        // Phase 2: Transfer the original comparison operator's trivia.
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
            if token_set![
                JS_FUNCTION_EXPRESSION,
                JS_CLASS_EXPRESSION,
                JS_OBJECT_EXPRESSION
            ]
            .contains(kind)
            {
                new_left = AnyJsExpression::from(make::parenthesized(new_left));
                // Re-check ASI: the generated `(` is itself a hazard when
                // preceded by a newline. e.g. `foo\n!function(){}===bar` →
                // `foo\n(function(){})!==bar` which parses as a call.
                if has_newline {
                    return None;
                }
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
