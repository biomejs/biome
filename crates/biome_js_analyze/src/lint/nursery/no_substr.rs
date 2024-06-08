use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{JsCallExpression, JsSyntaxToken};
use biome_rowan::{AstSeparatedList, BatchMutationExt, TextRange, TokenText};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce the use of `slice()` over `substr()` and `substring()`.
    ///
    /// `slice()` is preferred over `substr()` and `substring()` because it is a more popular option with clearer behavior,
    ///  and it has a consistent counterpart in arrays.
    ///
    /// Note that `substr`, `substring` and `slice` are not identical when arguments are passed.
    /// For detailed differences, refer to the MDN documentation:
    /// - [The difference between substring() and substr()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#the_difference_between_substring_and_substr)
    /// - [Differences between substring() and slice()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#differences_between_substring_and_slice)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foo.substr(start, length);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.substring(indexStart, indexEnd);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// foo.slice(beginIndex, endIndex);
    /// ```
    ///
    pub NoSubstr {
        version: "next",
        name: "noSubstr",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoSubstr {
    type Query = Ast<JsCallExpression>;
    type State = NoSubstrState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let expression = callee.as_js_static_member_expression()?;
        let value_token = expression.member().ok()?.value_token().ok()?;
        let string_function_name = value_token.text_trimmed();

        if matches!(string_function_name, "substr" | "substring") {
            let arguments = node.arguments().ok()?;
            let args = arguments.args();

            Some(NoSubstrState {
                value_token,
                has_arguments: !args.is_empty(),
            })
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic_message = markup! {
            "Avoid using "{state.member_name().text()}" and consider using slice instead."
        }
        .to_owned();
        let note_message = {
            markup! {
                "Use "<Emphasis>"."{state.member_name().text()}"()"</Emphasis>" instead of "<Emphasis>".slice()"</Emphasis>"."
            }
            .to_owned()
        };
        let mdn_link = {
            markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#the_difference_between_substring_and_substr">"MDN web docs"</Hyperlink>" for more details."
            }.to_owned()
        };
        Some(
            RuleDiagnostic::new(rule_category!(), state.span(), diagnostic_message)
                .note(note_message)
                .note(mdn_link),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if state.has_arguments {
            // If the function has arguments, we cannot replace it with slice() as it has different behavior.
            // - https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#differences_between_substring_and_slice
            // - https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substr#description
            return None;
        }

        let node = ctx.query();
        let callee = node.callee().ok()?;
        let expression = callee.as_js_static_member_expression()?;
        let member = expression.member().ok()?;

        let mut mutation = ctx.root().begin();
        let replaced_function = make::js_name(make::ident("slice"));
        mutation.replace_element(member.into(), replaced_function.into());

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Replace with "<Emphasis>".slice()"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct NoSubstrState {
    value_token: JsSyntaxToken,
    has_arguments: bool,
}

impl NoSubstrState {
    fn member_name(&self) -> TokenText {
        self.value_token.token_text_trimmed()
    }

    fn span(&self) -> TextRange {
        self.value_token.text_range()
    }
}
