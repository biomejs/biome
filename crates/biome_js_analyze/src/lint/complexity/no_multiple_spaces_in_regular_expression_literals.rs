use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsRegexLiteralExpression, JsSyntaxKind, JsSyntaxToken, TextRange, TextSize};
use biome_rowan::BatchMutationExt;
use std::{fmt::Write, ops::Range};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow unclear usage of consecutive space characters in regular expression literals
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
    /// /foo  */
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /foo  {2,}bar   {3,5}baz/
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
    /// / foo bar baz /
    ///```
    ///
    /// ```js
    /// /foo bar	baz/
    ///```
    pub NoMultipleSpacesInRegularExpressionLiterals {
        version: "1.0.0",
        name: "noMultipleSpacesInRegularExpressionLiterals",
        language: "js",
        sources: &[RuleSource::Eslint("no-regex-spaces")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoMultipleSpacesInRegularExpressionLiterals {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = Vec<Range<usize>>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let value_token = ctx.query().value_token().ok()?;
        let trimmed_text = value_token.text_trimmed();
        let mut range_list = vec![];
        let mut previous_is_space = false;
        let mut first_consecutive_space_index = 0;
        for (i, ch) in trimmed_text.bytes().enumerate() {
            if ch == b' ' {
                if !previous_is_space {
                    previous_is_space = true;
                    first_consecutive_space_index = i;
                }
            } else if previous_is_space {
                if i - first_consecutive_space_index > 1 {
                    range_list.push(first_consecutive_space_index..i);
                }
                previous_is_space = false;
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
        let Range {
            start: first_start, ..
        } = state[0];
        let Range { end: last_end, .. } = state[state.len() - 1];
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                TextRange::new(
                    value_token_range.start() + TextSize::from(first_start as u32),
                    value_token_range.start() + TextSize::from(last_end as u32),
                ),
                markup! {
                    "This regular expression contains unclear uses of consecutive spaces."
                },
            )
            .note(markup! { "It's hard to visually count the amount of spaces." }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let token = ctx.query().value_token().ok()?;
        let text = token.text_trimmed();
        let mut normalized_text = String::with_capacity(text.len());
        let mut previous_start = 0;
        for range in state {
            // copy previous characters and the first space
            normalized_text += &text[previous_start..range.start + 1];
            // handle quantifiers
            // See: https://262.ecma-international.org/#prod-QuantifierPrefix
            // `n` holds the number of characters used by the quantifier
            let n = match text.as_bytes().get(range.end) {
                Some(b'?') => {
                    write!(normalized_text, "{{{},{}}}", range.len() - 1, range.len()).unwrap();
                    1
                }
                Some(b'+') => {
                    write!(normalized_text, "{{{},}}", range.len()).unwrap();
                    1
                }
                Some(b'*') => {
                    if range.len() == 2 {
                        write!(normalized_text, "+").unwrap();
                    } else {
                        write!(normalized_text, "{{{},}}", range.len() - 1).unwrap();
                    }
                    1
                }
                Some(b'{') => {
                    if let Some((quantifier, n)) = parse_range_quantifier(&text[range.end..]) {
                        match quantifier {
                            RegexQuantifier::Amount(amount) => {
                                write!(normalized_text, "{{{}}}", amount + range.len() - 1)
                                    .unwrap();
                            }
                            RegexQuantifier::OpenRange(start) => {
                                write!(normalized_text, "{{{},}}", start + range.len() - 1)
                                    .unwrap();
                            }
                            RegexQuantifier::InclusiveRange((start, end)) => {
                                let extra = range.len() - 1;
                                write!(normalized_text, "{{{},{}}}", start + extra, end + extra)
                                    .unwrap();
                            }
                        }
                        n
                    } else {
                        // invalid range quantifiers are treated as regular chars
                        write!(normalized_text, "{{{}}}", range.len()).unwrap();
                        0
                    }
                }
                _ => {
                    write!(normalized_text, "{{{}}}", range.len()).unwrap();
                    0
                }
            };
            previous_start = range.end + n;
        }
        normalized_text += &text[previous_start..];
        let next_trimmed_token =
            JsSyntaxToken::new_detached(JsSyntaxKind::JS_REGEX_LITERAL, &normalized_text, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token(token, next_trimmed_token);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a quantifier instead." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug)]
enum RegexQuantifier {
    /// `{n}`
    Amount(usize),
    /// `{n,}`
    OpenRange(usize),
    /// `{n,m}`
    InclusiveRange((usize, usize)),
}

/// Returns the quantifier and the number of consumed characters,
/// if `source` starts with a well-formed range quantifier such as `{1,2}`.
fn parse_range_quantifier(source: &str) -> Option<(RegexQuantifier, usize)> {
    debug_assert!(source.starts_with('{'));
    let quantifier_end = source.find('}')?;
    let comma = source[..quantifier_end].find(',');
    // A range quantifier must include at least one number.
    // If a comma is present, a number must precede the comma.
    let quantifier_start: usize = source[1..comma.unwrap_or(quantifier_end)].parse().ok()?;
    let quantifier = if let Some(comma) = comma {
        debug_assert!(comma < quantifier_end);
        let quantifier_end = source[comma + 1..quantifier_end].parse::<usize>();
        if let Ok(quantifier_end) = quantifier_end {
            RegexQuantifier::InclusiveRange((quantifier_start, quantifier_end))
        } else {
            RegexQuantifier::OpenRange(quantifier_start)
        }
    } else {
        RegexQuantifier::Amount(quantifier_start)
    };
    Some((quantifier, quantifier_end + 1))
}
