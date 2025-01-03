use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsCallExpression, JsNewExpression, JsParenthesizedExpression, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TokenText, TriviaPieceKind};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Require `new` when throwing an error.
    ///
    /// While it's possible to instantiate `Error` without using the `new` keyword, it's better to be consistent: modern builtins require `new` to be instantiated.
    ///
    /// Rule matches errors when their name ends with the word "Error" and the first character is uppercase.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// throw Error();
    /// ```
    /// ```js,expect_diagnostic
    /// throw TypeError('biome');
    /// ```
    /// ```js,expect_diagnostic
    /// throw lib.TypeError();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// throw new Error();
    /// ```
    /// ```js
    /// throw new TypeError('biome');
    /// ```
    /// ```js
    /// throw new lib.TypeError();
    /// ```
    ///
    pub UseThrowNewError {
        version: "1.8.0",
        name: "useThrowNewError",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("throw-new-error")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseThrowNewError {
    type Query = Ast<JsCallExpression>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !is_in_throw_statement(node) {
            return None;
        }

        let callee = &node.callee().ok()?.omit_parentheses();
        let name = match callee {
            AnyJsExpression::JsIdentifierExpression(ident_expr) => Some(
                ident_expr
                    .name()
                    .ok()?
                    .value_token()
                    .ok()?
                    .token_text_trimmed(),
            ),
            AnyJsExpression::JsStaticMemberExpression(member_expr) => Some(
                member_expr
                    .member()
                    .ok()?
                    .value_token()
                    .ok()?
                    .token_text_trimmed(),
            ),
            _ => None,
        }?;

        if name.ends_with("Error") && name.chars().next()?.is_uppercase() {
            return Some(name);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let name = state.text();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Use "<Emphasis>"new "{name}"()"</Emphasis>" instead of "<Emphasis>{name}"()"</Emphasis>" when throwing an error."
            },
        ).note(markup! {
            "Instantiate "<Emphasis>"Error"</Emphasis>" with "<Emphasis>"new"</Emphasis>" keyword for consistency with modern builtins."
        }))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let new_expression = convert_call_expression_to_new_expression(node)?;

        mutation.replace_node::<AnyJsExpression>(node.clone().into(), new_expression.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add "<Emphasis>"new"</Emphasis>" keyword." }.to_owned(),
            mutation,
        ))
    }
}

fn does_member_contains_call_expression(expr: &AnyJsExpression) -> Option<bool> {
    let mut current_node = expr.clone();

    loop {
        current_node = current_node.omit_parentheses();

        if let Some(static_member_expr) = current_node.as_js_static_member_expression() {
            current_node = static_member_expr.object().ok()?.clone();
        } else if current_node.as_js_call_expression().is_some() {
            return Some(true);
        } else {
            return None;
        }
    }
}

pub(crate) fn convert_call_expression_to_new_expression(
    expr: &JsCallExpression,
) -> Option<JsNewExpression> {
    let mut callee = expr.callee().ok()?;
    let leading_trivia_pieces = callee.syntax().first_leading_trivia()?.pieces();

    // To use `new` keyword, we need to wrap the callee in parentheses if it contains a call expression.
    // Example: `foo().bar()` -> `new (foo().bar())`
    if !JsParenthesizedExpression::can_cast(callee.syntax().kind())
        && does_member_contains_call_expression(&callee).is_some()
    {
        callee = AnyJsExpression::JsParenthesizedExpression(make::js_parenthesized_expression(
            make::token(T!['(']),
            callee,
            make::token(T![')']),
        ));
    }

    let new_token = make::token(T![new])
        .with_leading_trivia_pieces(leading_trivia_pieces)
        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);

    callee = callee.with_leading_trivia_pieces([])?;

    Some(
        make::js_new_expression(new_token, callee)
            .with_arguments(expr.arguments().ok()?)
            .build(),
    )
}

/// Checks if the given expression is inside a `throw` statement.
fn is_in_throw_statement(expr: &JsCallExpression) -> bool {
    expr.syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| ancestor.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION)
        .is_some_and(|ancestor| ancestor.kind() == JsSyntaxKind::JS_THROW_STATEMENT)
}
