use std::{borrow::Cow, ops::Deref};

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, RuleSourceKind, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    JsNumberLiteralExpression, JsSyntaxToken, numbers::split_into_radix_and_number,
};
use biome_rowan::{AstNode, BatchMutationExt};
use serde::{Deserialize, Serialize};

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

        let num = NumericLiteral::parse(raw);
        let expected = num.format();

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
        let num = NumericLiteral::parse(token.text_trimmed());

        let new_token = JsSyntaxToken::new_detached(token.kind(), &num.format(), [], []);
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
        num.chars()
            .rev()
            .collect::<Vec<_>>()
            .chunks(group_length)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("_")
            .chars()
            .rev()
            .collect()
    }
}

/// Add chunk separators to a number string, starting from the left. Used for fractional parts.
/// The "uneven" chunk is added to the right of the last separator.
/// 12345654321 -> 123_456_543_21
fn add_separators_from_left(num: &str, min_digits: usize, group_length: usize) -> String {
    if num.len() < min_digits {
        num.to_owned()
    } else {
        num.chars()
            .collect::<Vec<_>>()
            .chunks(group_length)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("_")
    }
}

#[derive(Debug)]
struct NumericLiteral {
    sign: Option<char>,
    radix: u8,
    number: String,
    fraction: Option<String>,
    exponent: Option<Exponent>,
}

#[derive(Debug)]
struct Exponent {
    prefix: char,
    exponent: String,
    sign: Option<char>,
}

fn split_into_sign_and_number(num: &str) -> (Option<char>, &str) {
    if let Some(rest) = num.strip_prefix('-') {
        (Some('-'), rest)
    } else if let Some(rest) = num.strip_prefix('+') {
        (Some('+'), rest)
    } else {
        (None, num)
    }
}

impl NumericLiteral {
    fn parse(raw: &str) -> Self {
        let (sign, num_without_sign) = split_into_sign_and_number(raw);
        let (radix, num_without_radix) = split_into_radix_and_number(num_without_sign);

        let (num_without_exp, exponent) = if radix == 10 {
            let prefix = if num_without_radix.contains('e') {
                'e'
            } else {
                'E'
            };

            num_without_radix.split_once(prefix).map_or(
                (num_without_radix.clone(), None),
                |(rest, exponent)| {
                    let (sign, exponent) = split_into_sign_and_number(exponent);
                    (
                        Cow::Borrowed(rest),
                        Some(Exponent {
                            exponent: exponent.to_owned(),
                            sign,
                            prefix,
                        }),
                    )
                },
            )
        } else {
            (num_without_radix, None)
        };

        let (number, fraction) = match num_without_exp.split_once('.') {
            Some((number, fraction)) => (number, Some(fraction.to_owned())),
            None => (num_without_exp.deref(), None),
        };

        NumericLiteral {
            sign,
            radix,
            number: number.to_owned(),
            fraction,
            exponent,
        }
    }

    fn format(&self) -> String {
        let (min_digits, group_length) = match self.radix {
            2 => (0, 4),
            8 => (0, 4),
            10 => (5, 3),
            16 => (0, 2),
            _ => unreachable!(),
        };

        let number = add_separators_from_right(&self.number, min_digits, group_length);
        let fraction = self
            .fraction
            .as_ref()
            .map(|fraction| {
                format!(
                    ".{}",
                    add_separators_from_left(fraction, min_digits, group_length)
                )
            })
            .unwrap_or_default();

        let exponent = self
            .exponent
            .as_ref()
            .map(|exp| {
                format!(
                    "{}{}{}",
                    exp.prefix,
                    exp.sign.unwrap_or_empty(),
                    add_separators_from_right(&exp.exponent, min_digits, group_length)
                )
            })
            .unwrap_or_default();

        format!(
            "{}{}{}{}{}",
            match self.radix {
                2 => "0b",
                8 => "0o",
                10 => "",
                16 => "0x",
                _ => unreachable!(),
            },
            self.sign.unwrap_or_empty(),
            number,
            fraction,
            exponent
        )
    }
}

trait OptionalCharExt {
    fn unwrap_or_empty(&self) -> String;
}

impl OptionalCharExt for Option<char> {
    fn unwrap_or_empty(&self) -> String {
        match self {
            Some(ch) => ch.to_string(),
            None => String::new(),
        }
    }
}
