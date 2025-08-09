use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsNumberLiteralExpression, JsSyntaxToken};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_numeric_separators::UseNumericSeparatorsOptions;

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

        let expected = format_numeric_literal(raw);

        if raw == expected {
            None
        } else if raw.contains('_') {
            if expected.contains('_') {
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
        let num = format_numeric_literal(token.text_trimmed());

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

// Options for the minimum length of a number required before adding separators and the length of digit groups between separators, respectively.
const BINARY_OPTS: (usize, usize) = (0, 4);
const OCTAL_OPTS: (usize, usize) = (0, 4);
const DECIMAL_OPTS: (usize, usize) = (5, 3);
const HEXADECIMAL_OPTS: (usize, usize) = (0, 2);

/// Formats all parts of a numeric literal by adding separators between groups of digits when appropriate.
fn format_numeric_literal(raw: &str) -> String {
    let mut bytes = raw.bytes().peekable();
    let mut result = Vec::new();
    let mut current_num = Vec::new();

    let mut is_base_10 = true;
    let mut in_fraction = false;
    let mut prefix_parsed = false;

    let (mut min_digits, mut group_len) = DECIMAL_OPTS;

    while let Some(b) = bytes.next() {
        match b {
            b'_' => {}
            b'0' if !prefix_parsed && !in_fraction => {
                if let Some(&next) = bytes.peek() {
                    let opts = match next {
                        b'b' | b'B' => Some(BINARY_OPTS),
                        b'o' | b'O' | b'0'..=b'7' => Some(OCTAL_OPTS),
                        b'x' | b'X' => Some(HEXADECIMAL_OPTS),
                        _ => None,
                    };

                    if let Some(opts) = opts {
                        (min_digits, group_len) = opts;
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
                    min_digits,
                    group_len,
                ));
                result.push(b);
                current_num.clear();
                in_fraction = false;
            }
            b'.' => {
                result.extend(insert_separators(
                    &current_num,
                    false,
                    min_digits,
                    group_len,
                ));
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
        min_digits,
        group_len,
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
    min_digits: usize,
    group_len: usize,
) -> Vec<u8> {
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
