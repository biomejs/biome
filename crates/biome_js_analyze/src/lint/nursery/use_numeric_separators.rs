use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, RuleSourceKind, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsNumberLiteralExpression, JsSyntaxToken};
use biome_rowan::{AstNode, BatchMutationExt};

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
        version: "next",
        name: "useNumericSeparators",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("numeric-separators-style"), RuleSource::Clippy("unreadable_literal")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseNumericSeparators {
    type Query = Ast<JsNumberLiteralExpression>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let token = ctx.query().value_token().ok()?;
        let raw = token.text_trimmed();

        let expected = format_num(raw);

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

        match state {
            State::UnreadableLiteral => Some(RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup!("Long numeric literal lacks separators"),
            )),
            State::InconsistentGrouping => Some(RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup!("Inconsistent grouping of digits in numeric literal"),
            )),
            State::UnnecessaryGrouping => Some(RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup!("Unnecessary grouping of digits in numeric literal"),
            )),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let token = ctx.query().value_token().ok()?;
        let num = format_num(token.text_trimmed());

        let new_token = JsSyntaxToken::new_detached(token.kind(), &num, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(token.clone(), new_token);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            match state {
                State::UnreadableLiteral => markup! { "Add numeric separators" },
                State::InconsistentGrouping => markup! { "Fix numeric separator grouping" },
                State::UnnecessaryGrouping => markup! { "Remove unnecessary numeric separators" },
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

/// Add chunk separators to a number string, starting from the right.
/// The "uneven" chunk is added to the left of the first separator.
/// 1234567890 -> 1_234_567_890
fn add_separators_from_right(num: &str, min_digits: usize, group_length: usize) -> String {
    if num.len() < min_digits {
        num.to_owned()
    } else {
        let mut result = String::new();
        let mut count = 0;

        for c in num.chars().rev() {
            if count > 0 && count % group_length == 0 {
                result.push('_');
            }
            result.push(c);
            count += 1;
        }

        result.chars().rev().collect()
    }
}

/// Add chunk separators to a number string, starting from the left. Used for fractional parts.
/// The "uneven" chunk is added to the right of the last separator.
/// 12345654321 -> 123_456_543_21
fn add_separators_from_left(num: &str, min_digits: usize, group_length: usize) -> String {
    if num.len() < min_digits {
        num.to_owned()
    } else {
        let mut result = String::new();
        let mut count = 0;

        for c in num.chars() {
            if count > 0 && count % group_length == 0 {
                result.push('_');
            }
            result.push(c);
            count += 1;
        }

        result
    }
}

const BINARY_OPTS: (usize, usize) = (0, 4);
const OCTAL_OPTS: (usize, usize) = (0, 4);
const DECIMAL_OPTS: (usize, usize) = (5, 3);
const HEXADECIMAL_OPTS: (usize, usize) = (0, 2);

fn format_num(raw: &str) -> String {
    let mut chars = raw.chars().peekable();
    let mut result = String::new();
    let mut current_num = String::new();

    let mut is_base_10 = true;
    let mut in_fraction = false;
    let mut prefix_parsed = false;

    let (mut min_digits, mut group_len) = DECIMAL_OPTS;

    while let Some(c) = chars.next() {
        match c {
            '_' => continue,
            '0' if !prefix_parsed && !in_fraction => {
                if let Some(&next) = chars.peek() {
                    match next {
                        'b' | 'B' => {
                            (min_digits, group_len) = BINARY_OPTS;
                        }
                        'o' | 'O' | '0'..='7' => {
                            (min_digits, group_len) = OCTAL_OPTS;
                        }
                        'x' | 'X' => {
                            (min_digits, group_len) = HEXADECIMAL_OPTS;
                        }
                        _ => {
                            result.push(c);
                            continue;
                        }
                    }
                    result.push(c);
                    result.push(chars.next().unwrap());
                    is_base_10 = false;
                    prefix_parsed = true;
                }
            }
            'e' | 'E' if is_base_10 => {
                flush_current_num(
                    &mut result,
                    &current_num,
                    in_fraction,
                    min_digits,
                    group_len,
                );
                result.push(c);
                current_num.clear();
                in_fraction = false;
            }
            '.' => {
                flush_current_num(&mut result, &current_num, false, min_digits, group_len);
                result.push(c);
                current_num.clear();
                in_fraction = true;
            }
            '-' | '+' => result.push(c),
            _ if c.is_ascii_alphanumeric() => {
                prefix_parsed = true;
                current_num.push(c);
            }

            _ => panic!("unexpected character '{}'", c),
        }
    }

    flush_current_num(
        &mut result,
        &current_num,
        in_fraction,
        min_digits,
        group_len,
    );

    result
}

fn flush_current_num(
    result: &mut String,
    current: &str,
    left_aligned: bool,
    min_digits: usize,
    group_len: usize,
) {
    if left_aligned {
        result.push_str(&add_separators_from_left(current, min_digits, group_len));
    } else {
        result.push_str(&add_separators_from_right(current, min_digits, group_len));
    }
}
