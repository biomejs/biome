use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsStatement, JsArrowFunctionExpression, JsFunctionBody,
    JsReturnStatement, T,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::use_consistent_arrow_return::{
    UseConsistentArrowReturnOptions, UseConsistentArrowReturnStyle,
};

declare_lint_rule! {
    /// Enforce consistent arrow function bodies.
    ///
    /// This rule enforces the use of arrow functions with no body block when the function body consists of a single return statement.
    /// This rule does not report when:
    /// - the function body contains directives (e.g. `"use strict"`), or
    /// - the body (or its descendants) contain comments, or
    /// - the single `return` has no argument (`return;`).
    ///
    /// The fix wraps expressions in parentheses when required for correctness (e.g. object literals and sequence expressions).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    ///```js,expect_diagnostic
    /// const bar = () => {
    ///     return {
    ///         bar: {
    ///             foo: 1,
    ///             bar: 2,
    ///         }
    ///     };
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = () => 0;
    /// const bar = () => { "use strict"; return 1 }
    /// const baz = () => { /* intentional */ return x }
    /// const qux = () => ({ a: 1 })   // already concise with parens
    /// ```
    ///
    pub UseConsistentArrowReturn {
        version: "2.2.3",
        name: "useConsistentArrowReturn",
        language: "js",
        sources: &[RuleSource::Eslint("arrow-body-style").same()],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

pub enum State {
    AddBraces(AnyJsExpression),
    RemoveBraces(JsFunctionBody),
}

impl Rule for UseConsistentArrowReturn {
    type Query = Ast<JsArrowFunctionExpression>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = UseConsistentArrowReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let arrow = ctx.query();
        let options = ctx.options();
        let body = arrow.body().ok()?;

        match options.style {
            UseConsistentArrowReturnStyle::Always => {
                if let AnyJsFunctionBody::AnyJsExpression(expr) = body {
                    if expr.syntax().has_comments_descendants() {
                        return None;
                    }
                    return Some(State::AddBraces(expr));
                }
            }
            UseConsistentArrowReturnStyle::Never => {
                if let AnyJsFunctionBody::JsFunctionBody(body) = body {
                    if !body.directives().is_empty() || body.syntax().has_comments_descendants() {
                        return None;
                    }

                    if body.statements().len() == 1 {
                        let first_statement = body.statements().iter().next()?;
                        if let Some(return_statement) =
                            JsReturnStatement::cast(first_statement.into_syntax())
                            && return_statement.argument().is_some()
                        {
                            return Some(State::RemoveBraces(body));
                        }
                    }
                }
            }
            UseConsistentArrowReturnStyle::AsNeeded => match body {
                AnyJsFunctionBody::AnyJsExpression(expr) => {
                    if options.require_for_object_literal {
                        let mut expression = expr.clone();
                        if let Some(paren_expr) = expression.as_js_parenthesized_expression() {
                            expression = paren_expr.expression().ok()?;
                        }

                        if expression.as_js_object_expression().is_some() {
                            if expr.syntax().has_comments_descendants() {
                                return None;
                            }
                            return Some(State::AddBraces(expr));
                        }
                    }
                }
                AnyJsFunctionBody::JsFunctionBody(body) => {
                    if !body.directives().is_empty() || body.syntax().has_comments_descendants() {
                        return None;
                    }

                    if body.statements().len() == 1 {
                        let first_statement = body.statements().iter().next()?;
                        if let Some(return_statement) =
                            JsReturnStatement::cast(first_statement.into_syntax())
                            && let Some(arg) = return_statement.argument()
                        {
                            if arg.as_js_object_expression().is_some()
                                && options.require_for_object_literal
                            {
                                return None;
                            }
                            return Some(State::RemoveBraces(body));
                        }
                    }
                }
            },
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = match state {
            State::AddBraces(_) => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This arrow function body should be a block statement."
                },
            ),
            State::RemoveBraces(_) => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The body of this arrow function contains a single "<Emphasis>"return"</Emphasis>" statement."
                },
            ),
        };
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let arrow = ctx.query();

        match state {
            State::AddBraces(expr) => {
                let old_body = arrow.body().ok()?;
                let expr_to_return = if let Some(paren_expr) = expr.as_js_parenthesized_expression()
                {
                    paren_expr.expression().ok()?
                } else {
                    expr.clone()
                };

                let return_statement =
                    make::js_return_statement(make::token(T![return]).with_trailing_trivia([(
                        biome_js_syntax::TriviaPieceKind::Whitespace,
                        " ",
                    )]))
                    .with_argument(expr_to_return)
                    .with_semicolon_token(make::token(T![;]))
                    .build();

                let statement = arrow.syntax().ancestors().find_map(AnyJsStatement::cast)?;
                let mut base_indent = String::new();
                if let Some(trivia) = statement.syntax().first_leading_trivia() {
                    for piece in trivia.pieces().rev() {
                        if piece.is_newline() {
                            break;
                        }
                        if let Some(ws) = piece.as_whitespace() {
                            base_indent.insert_str(0, ws.text());
                        }
                    }
                }

                let body_indent =
                    format!("{base_indent}{}", ctx.preferred_indentation().to_string());
                let body = make::js_function_body(
                    make::token(T!['{']).with_trailing_trivia([
                        (biome_js_syntax::TriviaPieceKind::Newline, "\n"),
                        (biome_js_syntax::TriviaPieceKind::Whitespace, &body_indent),
                    ]),
                    make::js_directive_list([]),
                    make::js_statement_list([AnyJsStatement::from(return_statement)]),
                    make::token(T!['}']).with_leading_trivia([
                        (biome_js_syntax::TriviaPieceKind::Newline, "\n"),
                        (biome_js_syntax::TriviaPieceKind::Whitespace, &base_indent),
                    ]),
                );
                mutation.replace_node(old_body, AnyJsFunctionBody::from(body));
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Add braces to the arrow function body." }.to_owned(),
                    mutation,
                ))
            }
            State::RemoveBraces(body) => {
                let return_statement = body.statements().iter().next()?;
                let return_statement = JsReturnStatement::cast(return_statement.into_syntax())?;
                let return_argument = return_statement.argument()?;

                let new_body = if needs_parens_in_concise_body(&return_argument) {
                    AnyJsExpression::from(make::parenthesized(return_argument))
                } else {
                    return_argument
                };

                mutation.replace_node(
                    AnyJsFunctionBody::from(body.clone()),
                    AnyJsFunctionBody::AnyJsExpression(new_body),
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove the return statement." }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}

fn needs_parens_in_concise_body(expr: &AnyJsExpression) -> bool {
    use AnyJsExpression::*;
    matches!(
        expr,
        JsObjectExpression(_)
            | JsSequenceExpression(_)
            | TsAsExpression(_)
            | TsSatisfiesExpression(_)
            | TsTypeAssertionExpression(_)
    )
}
