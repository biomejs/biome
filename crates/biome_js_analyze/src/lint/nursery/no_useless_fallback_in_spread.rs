use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsLogicalExpression, JsLogicalOperator, JsObjectMemberList, JsSpread,
};
use biome_rowan::{AstNode, BatchMutationExt, Direction, TextRange};
use biome_rule_options::no_useless_fallback_in_spread::NoUselessFallbackInSpreadOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow empty object fallbacks when spreading a value into an object.
    ///
    /// Spreading `null`, `undefined`, or another falsy value (a value treated as `false` in a
    /// condition, such as `false`, `0`, or `""`) into an object literal adds no properties.
    /// Writing `{...(value || {})}` or `{...(value ?? {})}` therefore gives the same result as
    /// `{...value}`, and the `{}` fallback only adds noise.
    ///
    /// This doesn't apply to arrays: spreading `null` or `undefined` into an array or a function
    /// call throws an error, so `[...(value || [])]` is allowed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const object = { ...(options || {}) };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const object = { ...(options ?? {}) };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const object = { ...options };
    /// const withDefaults = { ...(options || { verbose: true }) };
    /// const conditional = { ...(enabled && {}) };
    /// const array = [...(items || [])];
    /// ```
    ///
    pub NoUselessFallbackInSpread {
        version: "2.6.7",
        name: "noUselessFallbackInSpread",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-useless-fallback-in-spread").same()],
        recommended: true,
        severity: Severity::Warning,
        // Unsafe because `document.all` is falsy but still has properties to spread.
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessFallbackInSpread {
    type Query = Ast<JsSpread>;
    /// The `value || {}` or `value ?? {}` expression being spread.
    type State = JsLogicalExpression;
    type Signals = Option<Self::State>;
    type Options = NoUselessFallbackInSpreadOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let spread = ctx.query();
        spread.parent::<JsObjectMemberList>()?;

        let logical = spread
            .argument()
            .ok()?
            .omit_parentheses()
            .as_js_logical_expression()?
            .clone();
        if logical.operator().ok()? == JsLogicalOperator::LogicalAnd {
            return None;
        }

        let right = logical.right().ok()?.omit_parentheses();
        right
            .as_js_object_expression()?
            .is_empty()
            .then_some(logical)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, logical: &Self::State) -> Option<RuleDiagnostic> {
        let range = TextRange::new(
            logical.operator_token().ok()?.text_trimmed_range().start(),
            logical.range().end(),
        );

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This empty object fallback is unnecessary."
                },
            )
            .note(markup! {
                "Spreading "<Emphasis>"null"</Emphasis>", "<Emphasis>"undefined"</Emphasis>", or another falsy value such as "<Emphasis>"false"</Emphasis>", "<Emphasis>"0"</Emphasis>", or "<Emphasis>"\"\""</Emphasis>" into an object adds no properties, so the fallback has no effect."
            })
            .note(markup! {
                "Remove the fallback and spread the value directly."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, logical: &Self::State) -> Option<JsRuleAction> {
        let argument = ctx.query().argument().ok()?;
        let left = logical.left().ok()?;
        if has_removed_comments(&argument, &left) {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(argument, left);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the empty object fallback." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` if replacing the spread `argument` with `left` would drop a comment.
///
/// The fix keeps the trivia before the first token and after the last token of `argument`,
/// and replaces the trivia at both ends of `left` with it. Comments anywhere else outside
/// `left`, such as next to the operator or inside the empty object, would be lost.
fn has_removed_comments(argument: &AnyJsExpression, left: &AnyJsExpression) -> bool {
    let argument = argument.syntax();
    let (Some(argument_first), Some(argument_last)) = (argument.first_token(), argument.last_token())
    else {
        return true;
    };
    let (Some(left_first), Some(left_last)) =
        (left.syntax().first_token(), left.syntax().last_token())
    else {
        return true;
    };
    let left_range = left.range();

    argument.descendants_tokens(Direction::Next).any(|token| {
        let is_inside_left = left_range.contains_range(token.text_trimmed_range());
        (token != argument_first
            && (!is_inside_left || token == left_first)
            && token.has_leading_comments())
            || (token != argument_last
                && (!is_inside_left || token == left_last)
                && token.has_trailing_comments())
    })
}
