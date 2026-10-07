use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsCallExpression, JsNewExpression, JsSyntaxToken, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, Direction, TextRange};
use biome_rule_options::no_useless_date_get_time::NoUselessDateGetTimeOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Disallow calling `.getTime()` on a date before passing it to the `Date` constructor.
    ///
    /// Passing a `Date` object to `new Date()` creates a copy of it.
    /// Calling `.getTime()` on the date first does the same thing with extra code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const copy = new Date(date.getTime());
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const copy = new Date(date);
    /// const timestamp = date.getTime();
    /// ```
    ///
    pub NoUselessDateGetTime {
        version: "next",
        name: "noUselessDateGetTime",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("consistent-date-clone").same()],
        recommended: true,
        severity: Severity::Warning,
        // Unsafe because `new Date(value)` and `new Date(value.getTime())` differ when `value` is not a `Date`.
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessDateGetTime {
    type Query = Semantic<JsNewExpression>;
    /// The `.getTime()` call passed to the `Date` constructor.
    type State = JsCallExpression;
    type Signals = Option<Self::State>;
    type Options = NoUselessDateGetTimeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let new_expression = ctx.query();
        let callee = new_expression
            .callee()
            .ok()?
            .omit_parentheses()
            .as_any_global_identifier_expression()?;
        let (reference, name) = global_identifier(&callee)?;
        if name.text() != "Date" {
            return None;
        }

        let arguments = new_expression.arguments()?.args();
        if arguments.len() != 1 {
            return None;
        }
        let call = arguments
            .first()?
            .ok()?
            .as_any_js_expression()?
            .clone()
            .omit_parentheses()
            .as_js_call_expression()?
            .clone();
        if call.is_optional_chain() || call.arguments().ok()?.args().len() != 0 {
            return None;
        }
        let member = call
            .callee()
            .ok()?
            .omit_parentheses()
            .as_js_static_member_expression()?
            .clone();
        if member.is_optional_chain()
            || member
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .text_trimmed()
                != "getTime"
        {
            return None;
        }

        ctx.model().binding(&reference).is_none().then_some(call)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, call: &Self::State) -> Option<RuleDiagnostic> {
        let member = call
            .callee()
            .ok()?
            .omit_parentheses()
            .as_js_static_member_expression()?
            .member()
            .ok()?;
        let range = TextRange::new(member.range().start(), call.range().end());

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>".getTime()"</Emphasis>" call is unnecessary."
                },
            )
            .note(markup! {
                "Passing a date directly to "<Emphasis>"new Date()"</Emphasis>" already creates a copy of it."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, call: &Self::State) -> Option<JsRuleAction> {
        let object = call
            .callee()
            .ok()?
            .omit_parentheses()
            .as_js_static_member_expression()?
            .object()
            .ok()?;
        let call_last_token = call.syntax().last_token()?;
        if has_removed_comments(call, &object, &call_last_token) {
            return None;
        }
        let object = object.append_trivia_pieces(call_last_token.trailing_trivia().pieces())?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(AnyJsExpression::from(call.clone()), object);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>".getTime()"</Emphasis>" call." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` if replacing `call` with `object` would drop a comment.
///
/// The trailing trivia of the call's last token is kept by the fix. Comments
/// after the object's last token are also rejected, because the fix would
/// otherwise remove the line break that ends a `//` comment.
fn has_removed_comments(
    call: &JsCallExpression,
    object: &AnyJsExpression,
    call_last_token: &JsSyntaxToken,
) -> bool {
    let object_range = object.range();
    object
        .syntax()
        .last_token()
        .is_some_and(|token| token.has_trailing_comments())
        || call
            .syntax()
            .descendants_tokens(Direction::Next)
            .filter(|token| !object_range.contains_range(token.text_trimmed_range()))
            .any(|token| {
                token.has_leading_comments()
                    || (&token != call_last_token && token.has_trailing_comments())
            })
}
