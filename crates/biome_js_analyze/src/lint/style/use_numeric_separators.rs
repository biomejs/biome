use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsNumberLiteralExpression, JsSyntaxToken};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_numeric_separators::{ResolvedOptions, UseNumericSeparatorsOptions};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce the use of numeric separators in numeric literals.
    ///
    /// Enforces a convention of grouping digits using [numeric separators](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#Numeric_separators).
    /// Long numbers can become difficult to read, so separating groups of digits with an underscore (`_`) improves code clarity. This rule also enforces proper usage of the numeric separator, by checking if the groups of digits are of the correct size.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1234567890;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = -999_99;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 0.1234567;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = 0b11001100;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = 1_234_567_890;
    /// ```
    ///
    /// ```js
    /// var a = -99_999;
    /// ```
    ///
    /// ```js
    /// var a = 0.123_456_7;
    /// ```
    ///
    /// ```js
    /// var a = 0b1100_1100;
    /// ```
    ///
    /// ## Options
    ///
    /// Each numeric literal type `binary`, `octal`, `decimal`, and `hexadecimal` can be configured with its own options object. Each type accepts `minimumDigits` or `groupLength`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "decimal": {
    ///             "minimumDigits": 7,
    ///             "groupLength": 3
    ///         },
    ///         "hexadecimal": {
    ///             "minimumDigits": 4,
    ///             "groupLength": 2
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### `<numeric literal type>.minimumDigits`
    ///
    /// Minimum number of digits required before adding separators. For example, with `minimumDigits` set to `5`, the number `1234` would not require separators, but `12345` would be expected to be formatted as `12_345`.
    ///
    /// Default values for `minimumDigits` vary by numeric literal type:
    /// - `binary`: `0`
    /// - `octal`: `0`
    /// - `decimal`: `5`
    /// - `hexadecimal`: `0`
    ///
    /// #### Invalid
    ///
    /// ```json,options
    /// {
    ///    "options": {
    ///       "decimal": {
    ///          "minimumDigits": 8
    ///       }
    ///    }
    /// }
    /// ```
    ///
    /// ```js,use_options,expect_diagnostic
    /// 12345678;
    /// ```
    ///
    /// #### Valid
    ///
    /// ```json,options
    /// {
    ///    "options": {
    ///       "decimal": {
    ///          "minimumDigits": 8
    ///       }
    ///    }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// 1234;
    /// 12_345_678;
    /// ```
    ///
    /// ### `<numeric literal type>.groupLength`
    ///
    /// Number of digits between separators. For example, with `groupLength` set to `2`, the number `12345` would be expected to be formatted as `1_23_45`.
    ///
    /// Default values for `groupLength` vary by numeric literal type:
    /// - `binary`: `4`
    /// - `octal`: `4`
    /// - `decimal`: `3`
    /// - `hexadecimal`: `2`
    ///
    /// #### Invalid
    ///
    /// ```json,options
    /// {
    ///    "options": {
    ///       "decimal": {
    ///          "groupLength": 5
    ///       }
    ///    }
    /// }
    /// ```
    ///
    /// ```js,use_options,expect_diagnostic
    /// 123_451_2345;
    /// ```
    ///
    /// #### Valid
    ///
    /// ```json,options
    /// {
    ///    "options": {
    ///       "decimal": {
    ///          "groupLength": 5
    ///       }
    ///    }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// 12345_12345;
    /// ```
    ///
    pub UseNumericSeparators {
        version: "2.0.0",
        name: "useNumericSeparators",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("numeric-separators-style").same(), RuleSource::Clippy("unreadable_literal").same()],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseNumericSeparators {
    type Query = Ast<JsNumberLiteralExpression>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = UseNumericSeparatorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let token = ctx.query().value_token().ok()?;
        let raw = token.text_trimmed();

        // Skip check for short numeric literals where separators don't improve readability.
        if raw.len() <= 3 {
            return None;
        }

        let analysis = analyze_numeric_literal(raw, ctx.options());

        if analysis.is_formatted {
            None
        } else if raw.contains('_') {
            if analysis.needs_separators {
                // Contains separators, but not in the same places as the expected value.
                Some(State::InconsistentGrouping)
            } else {
                // Contains separators which are not present in the expected value.
                Some(State::UnnecessaryGrouping)
            }
        } else {
            // Missing separators entirely.
            Some(State::UnreadableLiteral)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let (title, note) = match state {
            State::UnreadableLiteral => (
                markup!("Long numeric literal lacks separators."),
                markup!(
                    "Adding separators helps improve readability and clarity for long numbers."
                ),
            ),
            State::InconsistentGrouping => (
                markup!("Inconsistent grouping of digits in numeric literal."),
                markup!(
                    "Numbers with inconsistently placed separators can be misleading or confusing."
                ),
            ),
            State::UnnecessaryGrouping => (
                markup!("Unnecessary grouping of digits in numeric literal."),
                markup!("Short numbers don't require separators."),
            ),
        };

        Some(RuleDiagnostic::new(rule_category!(), node.range(), title).note(note))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let token = ctx.query().value_token().ok()?;
        let num = format_numeric_literal(token.text_trimmed(), ctx.options());

        let new_token = JsSyntaxToken::new_detached(token.kind(), &num, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(token.clone(), new_token);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            match state {
                State::UnreadableLiteral => markup! { "Add numeric separators." },
                State::InconsistentGrouping => {
                    markup! { "Use consistent numeric separator grouping." }
                }
                State::UnnecessaryGrouping => markup! { "Remove unnecessary numeric separators." },
            },
            mutation,
        ))
    }
}

pub enum State {
    UnreadableLiteral,
    InconsistentGrouping,
    UnnecessaryGrouping,
}

struct NumericLiteralAnalysis {
    is_formatted: bool,
    needs_separators: bool,
}

struct ChunkAnalysis {
    is_formatted: bool,
    needs_separators: bool,
}

/// Analyzes a numeric literal without allocating the formatted representation.
fn analyze_numeric_literal(
    raw: &str,
    options: &UseNumericSeparatorsOptions,
) -> NumericLiteralAnalysis {
    let bytes = raw.as_bytes();
    let mut current_num_start = 0;

    let mut is_formatted = true;
    let mut needs_separators = false;

    let mut is_base_10 = true;
    let mut in_fraction = false;
    let mut prefix_parsed = false;

    let mut separator_options = options.decimal();

    let mut index = 0;
    while let Some(&byte) = bytes.get(index) {
        match byte {
            b'_' => {}
            b'0' if !prefix_parsed && !in_fraction => {
                if let Some(&next) = bytes.get(index + 1) {
                    let opts = match next {
                        b'b' | b'B' => Some(options.binary()),
                        b'o' | b'O' | b'0'..=b'7' => Some(options.octal()),
                        b'x' | b'X' => Some(options.hexadecimal()),
                        _ => None,
                    };

                    if let Some(opts) = opts {
                        separator_options = opts;
                        is_base_10 = false;
                        prefix_parsed = true;
                        current_num_start = index + 2;
                        index += 1;
                    }
                }
            }
            b'e' | b'E' if is_base_10 => {
                let analysis = analyze_number_chunk(
                    &bytes[current_num_start..index],
                    in_fraction,
                    &separator_options,
                );
                is_formatted &= analysis.is_formatted;
                needs_separators |= analysis.needs_separators;
                current_num_start = index + 1;
                in_fraction = false;
            }
            b'.' => {
                let analysis = analyze_number_chunk(
                    &bytes[current_num_start..index],
                    false,
                    &separator_options,
                );
                is_formatted &= analysis.is_formatted;
                needs_separators |= analysis.needs_separators;
                current_num_start = index + 1;
                in_fraction = true;
            }
            b'-' | b'+' => {
                current_num_start = index + 1;
            }
            b if b.is_ascii_alphanumeric() => {
                prefix_parsed = true;
            }

            _ => panic!("unexpected byte '{byte}'",),
        }

        index += 1;
    }

    let analysis =
        analyze_number_chunk(&bytes[current_num_start..], in_fraction, &separator_options);
    is_formatted &= analysis.is_formatted;
    needs_separators |= analysis.needs_separators;

    NumericLiteralAnalysis {
        is_formatted,
        needs_separators,
    }
}

fn analyze_number_chunk(
    raw: &[u8],
    left_aligned: bool,
    separator_options: &ResolvedOptions,
) -> ChunkAnalysis {
    let min_digits = separator_options.minimum_digits as usize;
    let group_len = separator_options.group_length.get() as usize;
    let digit_count = raw.iter().filter(|&&byte| byte != b'_').count();
    let needs_separators = digit_count >= min_digits && digit_count > group_len;

    let mut current_digit = 0;
    let mut consumed_separator = false;
    let mut is_formatted = true;

    for &byte in raw {
        let should_have_separator = needs_separators
            && current_digit > 0
            && if left_aligned {
                current_digit % group_len == 0
            } else {
                (digit_count - current_digit) % group_len == 0
            };

        if byte == b'_' {
            if !needs_separators || !should_have_separator || consumed_separator {
                is_formatted = false;
                break;
            }

            consumed_separator = true;
            continue;
        }

        if should_have_separator && !consumed_separator {
            is_formatted = false;
            break;
        }

        consumed_separator = false;
        current_digit += 1;
    }

    if is_formatted && current_digit != digit_count {
        is_formatted = false;
    }

    ChunkAnalysis {
        is_formatted,
        needs_separators,
    }
}

/// Formats all parts of a numeric literal by adding separators between groups of digits when appropriate.
fn format_numeric_literal(raw: &str, options: &UseNumericSeparatorsOptions) -> String {
    let mut bytes = raw.bytes().peekable();
    let mut result = Vec::new();
    let mut current_num = Vec::new();

    let mut is_base_10 = true;
    let mut in_fraction = false;
    let mut prefix_parsed = false;

    let mut separator_options = options.decimal();

    while let Some(b) = bytes.next() {
        match b {
            b'_' => {}
            b'0' if !prefix_parsed && !in_fraction => {
                if let Some(&next) = bytes.peek() {
                    let opts = match next {
                        b'b' | b'B' => Some(options.binary()),
                        b'o' | b'O' | b'0'..=b'7' => Some(options.octal()),
                        b'x' | b'X' => Some(options.hexadecimal()),
                        _ => None,
                    };

                    if let Some(opts) = opts {
                        separator_options = opts;
                        result.push(b);
                        // SAFETY: We already peeked and confirmed the existence of the `next` byte, so we can safely advance the iterator and push the value immediately.
                        result.push(bytes.next().unwrap());
                        is_base_10 = false;
                        prefix_parsed = true;
                    } else {
                        result.push(b);
                    }
                }
            }
            b'e' | b'E' if is_base_10 => {
                result.extend(&insert_separators(
                    &current_num,
                    in_fraction,
                    &separator_options,
                ));
                result.push(b);
                current_num.clear();
                in_fraction = false;
            }
            b'.' => {
                result.extend(insert_separators(&current_num, false, &separator_options));
                result.push(b);
                current_num.clear();
                in_fraction = true;
            }
            b'-' | b'+' => result.push(b),
            b if b.is_ascii_alphanumeric() => {
                prefix_parsed = true;
                current_num.push(b);
            }

            _ => panic!("unexpected byte '{b}'",),
        }
    }

    result.extend(insert_separators(
        &current_num,
        in_fraction,
        &separator_options,
    ));

    // SAFETY: The original string is valid UTF-8 so we can be confident here that the result is valid as well.
    String::from_utf8(result).unwrap()
}

/// Add separators (potentially `left_aligned`) to the `number` and push to the `result` vec.
///
/// When right aligned, separators are added starting from the right (in reverse).
/// ```
/// 1234567; // -> 1_234_567
/// //             ^ The "uneven" group of numbers is to the left of the first separator.
/// ```
///
/// When left aligned, separators are added starting from the left.
/// ```
/// 12345654; // -> 123_456_54
/// //                      ^^ The "uneven" group of numbers is added to the right of the last separator.
/// ```
fn insert_separators(
    number: &[u8],
    left_aligned: bool,
    separator_options: &ResolvedOptions,
) -> Vec<u8> {
    let min_digits = separator_options.minimum_digits as usize;
    let group_len = separator_options.group_length.get() as usize;
    if number.len() < min_digits {
        // Don't insert separators if the number is shorter than the minimum number of digits required to start adding separators.
        number.to_vec()
    } else {
        // Iterate backwards if right aligned.
        let iter: Box<dyn Iterator<Item = &u8>> = if left_aligned {
            Box::new(number.iter())
        } else {
            Box::new(number.iter().rev())
        };

        let mut result: Vec<u8> = Vec::new();

        for (count, &b) in iter.enumerate() {
            if count > 0 && count % group_len == 0 {
                result.push(b'_');
            }
            result.push(b);
        }

        // Reverse the result if right aligned (to restore the correct order - original iterator was reversed while adding separators).
        if !left_aligned {
            result.reverse();
        }

        result
    }
}
