use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsCallExpression};
use biome_rowan::{AstSeparatedList, BatchMutationExt, NodeOrToken, TextRange};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce the use of `String.trimStart()` and `String.trimEnd()` over `String.trimLeft()` and `String.trimRight()`.
    ///
    /// While `String.trimLeft()` and `String.trimRight()` are aliases for `String.trimStart()` and `String.trimEnd()`,
    /// only using the latter pair ensures consistency and is preferable for their direction-independent wording.
    ///
    /// Note that `String.trimStart()` and `String.trimEnd()` methods do not take any parameters. Any arguments passed to these methods will be ignored.
    /// See the MDN documentation for more details:
    /// - [String.prototype.trimStart()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart)
    /// - [String.prototype.trimEnd()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd)
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
    member_name: String,
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
        let arguments = node.arguments().ok()?;
        let args = arguments.args();

        if !args.is_empty() {
            // If arguments are present, it suggests this function call may not be intended for `String.trimStart()` or `String.trimEnd()`,
            // as these methods do not accept parameters according to the specification:
            // - https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart#parameters
            // - https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd#parameters
            return None;
        }

        let callee = node.callee().ok()?;
        let (member_name, span, suggested_name) = match callee {
            AnyJsExpression::JsComputedMemberExpression(callee) => {
                let member = callee.member().ok()?;
                let value = member.as_static_value()?;
                let span = value.range();
                let member_name = value.as_string_constant()?.to_string();
                let suggested_name = match member_name.as_ref() {
                    "trimLeft" => Some("trimStart"),
                    "trimRight" => Some("trimEnd"),
                    _ => return None,
                };
                (member_name, span, suggested_name)
            }
            AnyJsExpression::JsStaticMemberExpression(callee) => {
                let token = callee.member().ok()?.value_token().ok()?;
                let span = token.text_range();
                let member_name = token.text_trimmed().to_string();
                let suggested_name = match member_name.as_ref() {
                    "trimLeft" => Some("trimStart"),
                    "trimRight" => Some("trimEnd"),
                    _ => return None,
                };
                (member_name, span, suggested_name)
            }
            _ => return None,
        };
        Some(UseTrimStartEndState {
            member_name,
            span,
            replaced_member_name: suggested_name?,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic_message = markup! {
            "Use "{state.replaced_member_name}" instead of "{state.member_name}"."
        }
        .to_owned();
        let note_message = {
            markup! {
                ""{state.member_name}"() is an alias for "{state.replaced_member_name}"."
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
        let token = match callee {
            AnyJsExpression::JsComputedMemberExpression(computed_expression) => computed_expression
                .member()
                .ok()?
                .get_callee_object_name()?,
            AnyJsExpression::JsStaticMemberExpression(static_expression) => {
                static_expression.member().ok()?.value_token().ok()?
            }
            _ => return None,
        };

        let replaced_member_name = state.replaced_member_name;
        let replaced_function = make::js_name(make::ident(&replaced_member_name));
        let mut mutation = ctx.root().begin();
        mutation.replace_element(NodeOrToken::Token(token), replaced_function.into());

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>{state.member_name}</Emphasis>" with "<Emphasis>{replaced_member_name}</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}
