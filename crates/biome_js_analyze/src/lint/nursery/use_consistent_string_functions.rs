use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::JsCallExpression;
use biome_rowan::{AstSeparatedList, BatchMutationExt, TextRange, TokenText};

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
    /// Note that `substring` and `slice` are not identical when arguments are passed.
    /// For detailed differences, refer to the MDN documentation:
    /// - [The difference between substring() and substr()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#the_difference_between_substring_and_substr)
    /// - [Differences between substring() and slice()](https:
    /// //developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#differences_between_substring_and_slice)
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

#[derive(Debug, Clone)]
pub struct UseConsistentStringFunctionsState {
    member_name: TokenText,
    span: TextRange,
}

impl Rule for UseConsistentStringFunctions {
    type Query = Ast<JsCallExpression>;
    type State = UseConsistentStringFunctionsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let expression = callee.as_js_static_member_expression()?;
        let value_token = expression.member().ok()?.value_token().ok()?;
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
        let member_name = state.member_name.text();
        let replaced_member_name = match member_name {
            "trimLeft" => "trimStart()",
            "trimRight" => "trimEnd()",
            "substr" | "substring" => "slice()",
            _ => return None,
        };
        let diagnostic_message = markup! {
            "Avoid using "{member_name}" and consider using "{replaced_member_name}" instead."
        }
        .to_owned();
        let note_message = if member_name == "substring" {
            markup! {
                "<Emphasis>{member_name}</Emphasis> and <Emphasis>{replaced_member_name}</Emphasis>differ in their behaviour when arguments are passed.
                ---
                See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#the_difference_between_substring_and_substr">"MDN web docs"</Hyperlink>" for more details."
            }.to_owned()
        } else if member_name == "substr" {
            markup! {
                "<Emphasis>{member_name}</Emphasis> and <Emphasis>{replaced_member_name}</Emphasis>differ in their behaviour particularly in the interpretation of the second argument and when start is greater than stop.
                ---
                See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substr#description">"MDN web docs"</Hyperlink>" for more details."
            }.to_owned()
        } else {
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

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let expression = callee.as_js_static_member_expression()?;
        let member = expression.member().ok()?;
        let expression = callee.as_js_static_member_expression()?;
        let value_token = expression.member().ok()?.value_token().ok()?;
        let member_name = value_token.text_trimmed();
        let arguments = node.arguments().ok()?;
        let args = arguments.args();

        let replaced_member_name = match member_name {
            "trimLeft" => "trimStart",
            "trimRight" => "trimEnd",
            "substr" | "substring" => "slice",
            _ => return None,
        };

        let mut mutation = ctx.root().begin();
        match member_name {
            "trimLeft" => {
                let replaced_function = make::js_name(make::ident(replaced_member_name));
                mutation.replace_element(member.into(), replaced_function.into());
            }
            "trimRight" => {
                let replaced_function = make::js_name(make::ident(replaced_member_name));
                mutation.replace_element(member.into(), replaced_function.into());
            }
            "substr" => {
                let replaced_function = make::js_name(make::ident(replaced_member_name));
                mutation.replace_element(member.into(), replaced_function.into());
            }
            "substring" => {
                if args.len() == 0 {
                    let replaced_function = make::js_name(make::ident("slice"));
                    let member = member.clone();
                    mutation.replace_element(member.into(), replaced_function.into());
                } else {
                    // If the function has arguments, we cannot replace it with slice() as it has different behavior.
                    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#differences_between_substring_and_slice
                    return None;
                }
            }
            _ => {}
        }

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Replace inconsistent string function "<Emphasis>{member_name}</Emphasis>" with "<Emphasis>{replaced_member_name}</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}
