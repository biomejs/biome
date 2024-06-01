use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::JsStaticMemberExpression;
use biome_rowan::{BatchMutationExt, TextRange, TokenText};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce the use of consistent string functions.
    ///
    /// This rule ensures that developers use consistent string functions by preferring:
    ///
    /// - `trimStart()` over `trimLeft()`,
    /// - `trimEnd()` over `trimRight()`,
    /// - `slice()` over `substr()` and `substring()`.
    ///
    /// While `trimLeft()` and `trimRight()` are aliases for `trimStart()` and `trimEnd()`, using the latter helps maintain consistency and uses direction-independent wording.
    /// Similarly, `slice()` is preferred over `substr()` and `substring()` because it is a more popular option with clearer behavior, and it has a consistent counterpart in arrays.
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
    /// const foo = bar.trimStart();
    /// ```
    ///
    /// ```js
    /// const foo = bar.trimEnd();
    /// ```
    ///
    /// ```js
    /// foo.slice(beginIndex, endIndex);
    /// ```
    ///
    pub UseConsistentStringFunctions {
        version: "next",
        name: "useConsistentStringFunctions",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

pub struct UseConsistentStringFunctionsState {
    member_name: TokenText,
    span: TextRange,
}

impl Rule for UseConsistentStringFunctions {
    type Query = Ast<JsStaticMemberExpression>;
    type State = UseConsistentStringFunctionsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let value_token = node.member().ok()?.value_token().ok()?;
        let string_function_name = value_token.text_trimmed();

        match string_function_name {
            "trimLeft" | "trimRight" | "substr" | "substring" => {
                Some(UseConsistentStringFunctionsState {
                    member_name: value_token.token_text_trimmed(),
                    span: value_token.text_range(),
                })
            }
            _ => None,
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = state.span;
        Some(RuleDiagnostic::new(
            rule_category!(),
            span,
            markup! {
                "Disallow using inconsistent string functions."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let member = node.member().ok()?;
        let member_name = state.member_name.text();
        let mut mutation = ctx.root().begin();

        let replaced_member_name = match member_name {
            "trimLeft" => "trimStart()",
            "trimRight" => "trimEnd()",
            "substr" | "substring" => "slice()",
            _ => return None,
        };

        match member_name {
            "trimLeft" => {
                let replaced_function = make::js_name(make::ident("trimStart"));
                mutation.replace_element(member.into(), replaced_function.into());
            }
            "trimRight" => {
                let replaced_function = make::js_name(make::ident("trimEnd"));
                mutation.replace_element(member.into(), replaced_function.into());
            }
            "substr" | "substring" => {
                let replaced_function = make::js_name(make::ident("slice"));
                mutation.replace_element(member.into(), replaced_function.into());
            }
            _ => {}
        }

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Replace inconsistent string function "<Emphasis>{member_name}</Emphasis>"()  with "<Emphasis>{replaced_member_name}</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}
