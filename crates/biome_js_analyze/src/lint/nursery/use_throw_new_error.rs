use std::ops::Not;

use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsCallExpression, JsNewExpression, JsParenthesizedExpression, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};
use lazy_static::lazy_static;
use regex::Regex;

use crate::JsRuleAction;

declare_rule! {
    /// Require new when throwing an error
    ///
    /// While it's possible to create a new error without using the new keyword, it's better to be explicit.
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
        version: "next",
        name: "useThrowNewError",
        sources: &[RuleSource::EslintUnicorn("throw-new-error")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseThrowNewError {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if is_in_throw_statement(node.clone()).not() {
            return None;
        }

        let callee = &node.callee().ok()?.omit_parentheses();
        let name = match callee {
            AnyJsExpression::JsIdentifierExpression(ident_expr) => Some(ident_expr.text()),
            AnyJsExpression::JsStaticMemberExpression(member_expr) => {
                let member_expr = member_expr.member().ok()?.text();

                Some(member_expr)
            }
            _ => None,
        }?;

        if ERROR_CONSTRUCTOR_REGEX.is_match(&name).not() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Use "<Emphasis>"new"</Emphasis>" when throwing an error."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let new_expression = convert_call_expression_to_new_expression(node)?;

        mutation.replace_node::<AnyJsExpression>(node.clone().into(), new_expression.into());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Add "<Emphasis>"new"</Emphasis>" keyword." }.to_owned(),
            mutation,
        })
    }
}

lazy_static! {
    static ref ERROR_CONSTRUCTOR_REGEX: Regex = Regex::new(r#"^(?:[A-Z][\da-z]*)*Error$"#).unwrap();
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

fn convert_call_expression_to_new_expression(expr: &JsCallExpression) -> Option<JsNewExpression> {
    let mut callee = expr.callee().ok()?;
    let leading_trivia_pieces = callee.syntax().first_leading_trivia()?.pieces();

    // To use `new` keyword, we need to wrap the callee in parentheses if it contains a call expression.
    // Example: `new foo.bar()` -> `new (foo.bar())`
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
fn is_in_throw_statement(expr: JsCallExpression) -> bool {
    let mut current_expr = expr.into_syntax();

    while let Some(parent) = current_expr.parent() {
        match parent.kind() {
            JsSyntaxKind::JS_THROW_STATEMENT => return true,
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => current_expr = parent,
            _ => return false,
        }
    }

    false
}
