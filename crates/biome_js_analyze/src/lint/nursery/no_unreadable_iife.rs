use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsStatement, JsCallExpression,
    JsParenthesizedExpression, T, TriviaPieceKind,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_unreadable_iife::NoUnreadableIifeOptions;

declare_lint_rule! {
    /// Disallow immediately called arrow functions whose body is wrapped in parentheses.
    ///
    /// An immediately invoked function expression, or IIFE, is a function that is called
    /// right where it is defined, such as `(() => value)()`.
    ///
    /// When the body of such an arrow function is also wrapped in parentheses, the code
    /// contains several nested pairs of parentheses. This makes the function look like
    /// a grouped expression and hides where the function body starts and ends.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = (bar => (bar ? bar.baz : baz))(getBar());
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = ((bar, baz) => ({ bar, baz }))(bar, baz);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const bar = getBar();
    /// const foo = bar ? bar.baz : baz;
    ///
    /// const getBaz = bar => (bar ? bar.baz : baz);
    /// const qux = getBaz(getBar());
    ///
    /// const quux = (bar => {
    ///     return bar ? bar.baz : baz;
    /// })(getBar());
    /// ```
    ///
    pub NoUnreadableIife {
        version: "next",
        name: "noUnreadableIife",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-unreadable-iife").same()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUnreadableIife {
    type Query = Ast<JsCallExpression>;
    /// The parenthesized body of the called arrow function.
    type State = JsParenthesizedExpression;
    type Signals = Option<Self::State>;
    type Options = NoUnreadableIifeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let callee = ctx.query().callee().ok()?.omit_parentheses();
        let arrow = callee.as_js_arrow_function_expression()?;
        let AnyJsFunctionBody::AnyJsExpression(AnyJsExpression::JsParenthesizedExpression(body)) =
            arrow.body().ok()?
        else {
            return None;
        };
        Some(body)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, body: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                body.range(),
                markup! {
                    "This immediately called arrow function has a body wrapped in parentheses."
                },
            )
            .note(markup! {
                "The extra parentheses make the function look like a grouped expression, which hides where the function body starts."
            })
            .note(markup! {
                "Move the code out of the function, or use a body in braces with a "<Emphasis>"return"</Emphasis>" statement."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, body: &Self::State) -> Option<JsRuleAction> {
        let expression = unwrap_parenthesized_body(body)?;
        let l_paren = body.l_paren_token().ok()?;
        let r_paren = body.r_paren_token().ok()?;

        let return_statement =
            make::js_return_statement(make::token(T![return]).with_trailing_space())
                .with_argument(expression.trim_trivia()?)
                .with_semicolon_token(make::token(T![;]))
                .build();
        let new_body = make::js_function_body(
            make::token(T!['{'])
                .with_leading_trivia_pieces(l_paren.leading_trivia().pieces())
                .with_trailing_space(),
            make::js_directive_list([]),
            make::js_statement_list([AnyJsStatement::from(return_statement)]),
            make::token(T!['}'])
                .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                .with_trailing_trivia_pieces(r_paren.trailing_trivia().pieces()),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsFunctionBody::AnyJsExpression(body.clone().into()),
            AnyJsFunctionBody::from(new_body),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a body in braces with a "<Emphasis>"return"</Emphasis>" statement." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Returns the expression inside every pair of parentheses around `body`.
///
/// Returns `None` when a comment sits between the parentheses or right before or after
/// that expression. Removing the parentheses would delete comments between them, and a
/// line comment at either end of the expression would comment out or end the `return`
/// statement.
fn unwrap_parenthesized_body(body: &JsParenthesizedExpression) -> Option<AnyJsExpression> {
    let mut parenthesized = body.clone();
    loop {
        if parenthesized.l_paren_token().ok()?.has_trailing_comments()
            || parenthesized.r_paren_token().ok()?.has_leading_comments()
        {
            return None;
        }
        match parenthesized.expression().ok()? {
            AnyJsExpression::JsParenthesizedExpression(inner) => {
                if inner.syntax().has_leading_comments() || inner.syntax().has_trailing_comments()
                {
                    return None;
                }
                parenthesized = inner;
            }
            expression => {
                if expression.syntax().has_leading_comments()
                    || expression.syntax().has_trailing_comments()
                {
                    return None;
                }
                return Some(expression);
            }
        }
    }
}
