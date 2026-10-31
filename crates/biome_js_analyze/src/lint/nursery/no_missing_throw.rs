use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, AnyJsStatement, JsExpressionStatement, T};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_missing_throw::NoMissingThrowOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow creating an error with `new` without throwing it.
    ///
    /// Writing `new Error()` as a statement, without using its result, creates
    /// an error object and then discards it. Nothing happens: the code keeps running as if no error
    /// occurred. This is almost always a mistake where the `throw` keyword was
    /// forgotten.
    ///
    /// The rule checks constructors whose name starts with an uppercase letter
    /// and ends with `Error`, such as `Error`, `TypeError`, or `ValidationError`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function check(value) {
    ///     if (!value) {
    ///         new Error("Missing value");
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const fail = () => {
    ///     new TypeError("Unexpected type");
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function check(value) {
    ///     if (!value) {
    ///         throw new Error("Missing value");
    ///     }
    /// }
    ///
    /// const createError = () => new Error("Something went wrong");
    ///
    /// const error = new Error("Something went wrong");
    ///
    /// reject(new Error("Something went wrong"));
    /// ```
    ///
    pub NoMissingThrow {
        version: "next",
        name: "noMissingThrow",
        language: "js",
        sources: &[RuleSource::Oxc("missing-throw").inspired()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoMissingThrow {
    type Query = Ast<JsExpressionStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoMissingThrowOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query().expression().ok()?.omit_parentheses();
        let new_expression = expression.as_js_new_expression()?;
        let name = match new_expression.callee().ok()?.omit_parentheses() {
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                identifier.name().ok()?.value_token().ok()?
            }
            AnyJsExpression::JsStaticMemberExpression(member) => {
                member.member().ok()?.value_token().ok()?
            }
            _ => return None,
        };
        let name = name.text_trimmed();

        (name.ends_with("Error") && name.as_bytes()[0].is_ascii_uppercase()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let expression = ctx.query().expression().ok()?.omit_parentheses();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                expression.range(),
                markup! {
                    "This error is created but never thrown."
                },
            )
            .note(markup! {
                "Creating an error does nothing by itself, so the code keeps running as if nothing went wrong."
            })
            .note(markup! {
                "Add the "<Emphasis>"throw"</Emphasis>" keyword to throw the error, or remove the expression if it isn't needed."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let statement = ctx.query();
        let expression = statement.expression().ok()?;
        let leading_trivia = expression.syntax().first_leading_trivia()?;

        let throw_token = make::token(T![throw])
            .with_leading_trivia_pieces(leading_trivia.pieces())
            .with_trailing_space();
        let mut throw_statement =
            make::js_throw_statement(throw_token, expression.with_leading_trivia_pieces([])?);
        if let Some(semicolon) = statement.semicolon_token() {
            throw_statement = throw_statement.with_semicolon_token(semicolon);
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node::<AnyJsStatement>(
            statement.clone().into(),
            throw_statement.build().into(),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add the "<Emphasis>"throw"</Emphasis>" keyword." },
            mutation,
        ))
    }
}
