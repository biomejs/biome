use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::JsStringLiteralExpression;
use biome_rowan::{AstNode, BatchMutationExt, TextRange};
use rustc_hash::FxHashSet;
use std::ops::Range;

declare_lint_rule! {
    /// Disallow `\8` and `\9` escape sequences in string literals.
    ///
    /// Since ECMAScript 2021, the escape sequences \8 and \9 have been defined as non-octal decimal escape sequences.
    /// However, most JavaScript engines consider them to be "useless" escapes. For example:
    ///
    /// ```js,ignore
    /// "\8" === "8"; // true
    /// "\9" === "9"; // true
    /// ```
    ///
    /// Although this syntax is deprecated, it is still supported for compatibility reasons.
    /// If the ECMAScript host is not a web browser, this syntax is optional.
    /// However, web browsers are still required to support it, but only in non-strict mode.
    /// Regardless of your targeted environment, it is recommended to avoid using these escape sequences in new code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const x = "\8";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const x = "Don't use \8 escape.";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const x = "Don't use \9 escape.";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const x = "8";
    /// ```
    ///
    /// ```js
    /// const x = "Don't use \\8 and \\9 escapes.";
    /// ```
    ///
    pub NoNonoctalDecimalEscape {
        version: "1.0.0",
        name: "noNonoctalDecimalEscape",
        language: "js",
        sources: &[RuleSource::Eslint("no-nonoctal-decimal-escape")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug)]
pub enum FixSuggestionKind {
    Refactor,
}

#[derive(Debug)]
pub struct RuleState {
    kind: FixSuggestionKind,
    diagnostics_text_range: TextRange,
    replace_from: Box<str>,
    replace_to: Box<str>,
    replace_string_range: Range<usize>,
}

impl Rule for NoNonoctalDecimalEscape {
    type Query = Ast<JsStringLiteralExpression>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut result = Vec::new();
        let Some(token) = node.value_token().ok() else {
            return result.into_boxed_slice();
        };
        let text = token.text();
        if !is_octal_escape_sequence(text) {
            return result.into_boxed_slice();
        }
        let matches = lex_escape_sequences(text);

        for EscapeSequence {
            previous_escape,
            decimal_escape,
            decimal_escape_range: (decimal_escape_string_start, decimal_escape_string_end),
        } in matches.iter()
        {
            let text_range_start = usize::from(node.range().start());
            let decimal_escape_range_start = text_range_start + decimal_escape_string_start;
            let decimal_escape_range_end = decimal_escape_range_start + decimal_escape.len();
            let Some(decimal_escape_range) =
                TextRange::try_from((decimal_escape_range_start, decimal_escape_range_end)).ok()
            else {
                continue;
            };

            let Some(decimal_char) = decimal_escape.chars().nth(1) else {
                continue;
            };

            let replace_string_range = *decimal_escape_string_start..*decimal_escape_string_end;

            if let Some(previous_escape) = previous_escape {
                if *previous_escape == "\\0" {
                    if let Some(unicode_escape) = get_unicode_escape('\0') {
                        let Some(previous_escape_range_start) = text.find(previous_escape) else {
                            continue;
                        };
                        let Some(unicode_escape_text_range) = TextRange::try_from((
                            text_range_start + previous_escape_range_start,
                            decimal_escape_range_end,
                        ))
                        .ok() else {
                            continue;
                        };

                        let replace_string_range =
                            previous_escape_range_start..*decimal_escape_string_end;

                        // \0\8 -> \u00008
                        result.push(RuleState {
                            kind: FixSuggestionKind::Refactor,
                            diagnostics_text_range: unicode_escape_text_range,
                            replace_from: format!("{previous_escape}{decimal_escape}").into(),
                            replace_to: format!("{unicode_escape}{decimal_char}").into(),
                            replace_string_range,
                        });
                    }

                    let Some(decimal_char_unicode_escaped) = get_unicode_escape(decimal_char)
                    else {
                        continue;
                    };
                    // \8 -> \u0038
                    result.push(RuleState {
                        kind: FixSuggestionKind::Refactor,
                        diagnostics_text_range: decimal_escape_range,
                        replace_from: decimal_escape.clone().into_boxed_str(),
                        replace_to: decimal_char_unicode_escaped.into_boxed_str(),
                        replace_string_range,
                    });
                } else {
                    // \8 -> 8
                    result.push(RuleState {
                        kind: FixSuggestionKind::Refactor,
                        diagnostics_text_range: decimal_escape_range,
                        replace_from: decimal_escape.clone().into_boxed_str(),
                        replace_to: decimal_char.to_string().into_boxed_str(),
                        replace_string_range,
                    })
                }
            }
        }

        let mut seen = FxHashSet::default();
        result.retain(|rule_state| seen.insert(rule_state.diagnostics_text_range));
        result.into_boxed_slice()
    }

    fn diagnostic(
        _: &RuleContext<Self>,
        RuleState {
            diagnostics_text_range,
            ..
        }: &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            diagnostics_text_range,
            markup! {
                "Don't use "<Emphasis>"`\\8`"</Emphasis>" and "<Emphasis>"`\\9`"</Emphasis>" escape sequences in string literals."
            },
        ).note(
			markup! {
				"The nonoctal decimal escape is a deprecated syntax that is left for compatibility and should not be used."
			}
		))
    }

    fn action(
        ctx: &RuleContext<Self>,
        RuleState {
            kind,
            replace_from,
            replace_to,
            replace_string_range,
            ..
        }: &Self::State,
    ) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();
        let prev_token = node.value_token().ok()?;
        let replaced = safe_replace_by_range(
            prev_token.text().to_string(),
            replace_string_range.clone(),
            replace_to,
        )?;

        let next_token = make::ident(&replaced);

        mutation.replace_token(prev_token, next_token);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
             match kind {
				FixSuggestionKind::Refactor => {
					markup! ("Replace "<Emphasis>{replace_from.as_ref()}</Emphasis>" with "<Emphasis>{replace_to.as_ref()}</Emphasis>". This maintains the current functionality.").to_owned()
				}
			},
            mutation,
        ))
    }
}

fn safe_replace_by_range(
    mut target: String,
    range: Range<usize>,
    replace_with: &str,
) -> Option<String> {
    debug_assert!(target.len() >= range.end, "Range out of bounds");
    debug_assert!(
        target.is_char_boundary(range.start) && target.is_char_boundary(range.end),
        "Range does not fall on char boundary"
    );
    target.replace_range(range, replace_with);
    Some(target)
}

/// Returns true if input is octal decimal escape sequence and is not in JavaScript regular expression
fn is_octal_escape_sequence(input: &str) -> bool {
    let mut in_regex = false;
    let mut prev_char_was_escape = false;
    for ch in input.chars() {
        match ch {
            '/' if !prev_char_was_escape => in_regex = !in_regex,
            '8' | '9' if prev_char_was_escape && !in_regex => return true,
            '\\' => prev_char_was_escape = !prev_char_was_escape,
            _ => prev_char_was_escape = false,
        }
    }
    false
}

#[derive(Debug, PartialEq)]
struct EscapeSequence {
    previous_escape: Option<String>,
    decimal_escape: String,
    /// The range of the decimal escape sequence in the string literal
    decimal_escape_range: (usize, usize),
}

/// Returns a list of escape sequences in the given string literal
fn lex_escape_sequences(input: &str) -> Vec<EscapeSequence> {
    let mut result = Vec::new();
    let mut previous_escape = None;
    let mut decimal_escape_start = None;
    let mut chars = input.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        match ch {
            '\\' => match chars.peek() {
                Some((_, '0')) => {
                    previous_escape = Some("\\0".to_string());
                    // Consume '0'
                    let _ = chars.next();
                }
                Some((_, '8'..='9')) => {
                    decimal_escape_start = Some(i);
                }
                _ => (),
            },
            '8' | '9' if decimal_escape_start.is_some() => {
                result.push(EscapeSequence {
                    previous_escape: previous_escape.take(),
                    decimal_escape: match ch {
                        '8' => "\\8".to_string(),
                        '9' => "\\9".to_string(),
                        _ => unreachable!(),
                    },
                    // SAFETY: We tested `decimal_escape_start.is_some()`
                    decimal_escape_range: (decimal_escape_start.unwrap(), i + ch.len_utf8()),
                });
                decimal_escape_start = None;
            }
            _ => previous_escape = Some(ch.to_string()),
        }
    }
    result
}

/// Returns unicode escape sequence "\uXXXX" that represents the given character
pub(crate) fn get_unicode_escape(ch: char) -> Option<String> {
    Some(format!("\\u{:04x}", ch as u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_octal_escape_sequence() {
        assert!(!is_octal_escape_sequence(""));
        assert!(!is_octal_escape_sequence("Hello World!"));
        assert!(!is_octal_escape_sequence("\\0"));
        assert!(!is_octal_escape_sequence("\\7"));
        assert!(is_octal_escape_sequence("\\8"));
        assert!(is_octal_escape_sequence("\\9"));
        assert!(!is_octal_escape_sequence("/\\8/"));
        assert!(!is_octal_escape_sequence("/\\9/"));
        assert!(is_octal_escape_sequence("\\0\\8"));
        assert!(is_octal_escape_sequence("\\7\\9"));
    }

    #[test]
    fn test_get_unicode_escape() {
        assert_eq!(get_unicode_escape('\0'), Some("\\u0000".to_string()));
        assert_eq!(get_unicode_escape('a'), Some("\\u0061".to_string()));
        assert_eq!(get_unicode_escape('👍'), Some("\\u1f44d".to_string()));
    }

    #[test]
    fn test_parse_escape_sequences() {
        assert_eq!(
            lex_escape_sequences("test\\8\\9"),
            vec![
                EscapeSequence {
                    previous_escape: Some("t".to_string()),
                    decimal_escape: "\\8".to_string(),
                    decimal_escape_range: (4, 6)
                },
                EscapeSequence {
                    previous_escape: None,
                    decimal_escape: "\\9".to_string(),
                    decimal_escape_range: (6, 8)
                }
            ]
        );
        assert_eq!(
            lex_escape_sequences("\\0\\8"),
            vec![EscapeSequence {
                previous_escape: Some("\\0".to_string()),
                decimal_escape: "\\8".to_string(),
                decimal_escape_range: (2, 4)
            },]
        );
        assert_eq!(
            lex_escape_sequences("👍\\8\\9"),
            vec![
                EscapeSequence {
                    previous_escape: Some("👍".to_string()),
                    decimal_escape: "\\8".to_string(),
                    decimal_escape_range: (4, 6)
                },
                EscapeSequence {
                    previous_escape: None,
                    decimal_escape: "\\9".to_string(),
                    decimal_escape_range: (6, 8)
                }
            ]
        );
        assert_eq!(
            lex_escape_sequences("\\\\ \\8"),
            vec![EscapeSequence {
                previous_escape: Some(" ".to_string()),
                decimal_escape: "\\8".to_string(),
                decimal_escape_range: (3, 5)
            },]
        )
    }
}
