use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::JsCallExpression;
use biome_rowan::{BatchMutationExt, TextRange, TokenText};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce the use of `String.trimStart()` and `String.trimEnd()` over `String.trimLeft()` and `String.trimRight()`.
    ///
    /// While `String.trimLeft()` and `String.trimRight()` are aliases for `String.trimStart()` and `String.trimEnd()`,
    /// only using the latter pair ensures consistency and is preferable for their direction-independent wording.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = bar.trimLeft();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = bar.trimRight();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = bar.trimStart();
    /// ```
    ///
    /// ```js
    /// const foo = bar.trimEnd();
    /// ```
    ///
    pub UseTrimStartEnd {
        version: "next",
        name: "useTrimStartEnd",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-string-trim-start-end")],
        fix_kind: FixKind::Safe,
    }
}

#[derive(Debug, Clone)]
pub struct UseTrimStartEndState {
    member_name: TokenText,
    span: TextRange,
    replaced_member_name: &'static str,
}

impl Rule for UseTrimStartEnd {
    type Query = Ast<JsCallExpression>;
    type State = UseTrimStartEndState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let expression = callee.as_js_static_member_expression()?;
        let value_token = expression.member().ok()?.value_token().ok()?;
        let name = value_token.text_trimmed();
        let suggested_name = match name {
            "trimLeft" => Some("trimStart"),
            "trimRight" => Some("trimEnd"),
            _ => None,
        }?;

        Some(UseTrimStartEndState {
            member_name: value_token.token_text_trimmed(),
            span: value_token.text_range(),
            replaced_member_name: suggested_name,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let member_name = state.member_name.text();
        let replaced_member_name = state.replaced_member_name;
        let diagnostic_message = markup! {
            "Use "{replaced_member_name}" instead of "{member_name}"."
        }
        .to_owned();
        let note_message = {
            markup! {
                ""{member_name}"() is an alias for "{replaced_member_name}"."
            }
            .to_owned()
        };
        Some(
            RuleDiagnostic::new(rule_category!(), state.span, diagnostic_message)
                .note(note_message),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let expression = callee.as_js_static_member_expression()?;
        let member = expression.member().ok()?;
        let member_name = state.member_name.text();
        let replaced_member_name = state.replaced_member_name;

        let mut mutation = ctx.root().begin();
        let replaced_function = make::js_name(make::ident(replaced_member_name));
        mutation.replace_element(member.into(), replaced_function.into());

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>{member_name}</Emphasis>" with "<Emphasis>{replaced_member_name}</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}
