use biome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsRegexLiteralExpression, JsSyntaxKind, JsSyntaxToken, TextRange, TextSize};
use rome_rowan::BatchMutationExt;
use std::fmt::Write;

use crate::JsRuleAction;

declare_rule! {
    /// Disallow unclear usage of multiple space characters in regular expression literals
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// /   /
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /  foo/
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /foo   /
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /foo  bar/
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /foo   bar    baz/
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /foo [ba]r  b(a|z)/
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// /foo {2}bar/
    ///```
    ///
    /// ```js
    /// /foo bar baz/
    ///```
    ///
    /// ```js
    /// /foo bar	baz/
    ///```
    ///
    /// ```js
    /// /foo /
    ///```
    pub(crate) NoMultipleSpacesInRegularExpressionLiterals {
        version: "1.0.0",
        name: "noMultipleSpacesInRegularExpressionLiterals",
        recommended: true,
    }
}

impl Rule for NoMultipleSpacesInRegularExpressionLiterals {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = Vec<(usize, usize)>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let value_token = ctx.query().value_token().ok()?;
        let trimmed_text = value_token.text_trimmed();
        let mut range_list = vec![];
        let mut continue_white_space = false;
        let mut last_white_index = 0;
        for (i, ch) in trimmed_text.chars().enumerate() {
            if ch == ' ' {
                if !continue_white_space {
                    continue_white_space = true;
                    last_white_index = i;
                }
            } else if continue_white_space {
                if i - last_white_index > 1 {
                    range_list.push((last_white_index, i));
                }
                continue_white_space = false;
            }
        }
        if !range_list.is_empty() {
            Some(range_list)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let value_token = ctx.query().value_token().ok()?;
        let value_token_range = value_token.text_trimmed_range();
        // SAFETY: We know diagnostic will be sended only if the `range_list` is not empty
        // first and last continuous whitespace range of `range_list`
        let (first_start, _) = state[0];
        let (_, last_end) = state[state.len() - 1];

        Some(RuleDiagnostic::new(
            rule_category!(),
            TextRange::new(
                value_token_range.start() + TextSize::from(first_start as u32),
                value_token_range.start() + TextSize::from(last_end as u32),
            ),
            markup! {
                "This regular expression contains unclear uses of multiple spaces."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let trimmed_token = ctx.query().value_token().ok()?;
        let trimmed_token_string = trimmed_token.text_trimmed();
        let mut normalized_string_token = String::new();
        let mut previous_start = 0;

        let mut eg_length = 0;

        for (start, end) in state.iter() {
            normalized_string_token += &trimmed_token_string[previous_start..*start];
            write!(normalized_string_token, " {{{}}}", *end - *start).unwrap();
            previous_start = *end;
            eg_length += *end - *start;
        }
        normalized_string_token += &trimmed_token_string[previous_start..];
        let next_trimmed_token = JsSyntaxToken::new_detached(
            JsSyntaxKind::JS_REGEX_LITERAL,
            &normalized_string_token,
            [],
            [],
        );
        mutation.replace_token(trimmed_token, next_trimmed_token);
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {"{eg_length}"}/" }.to_owned(),
            mutation,
        })
    }
}
