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
    /// a = -999_99;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = 1_234_567_890;
    /// a = -99_999;
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
    type Options = UseNumericSeparatorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let token = ctx.query().value_token().ok()?;
        let raw = token.text_trimmed();

        let num = NumericLiteral::parse(raw);
        let expected = num.format(ctx.options().clone());

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

        let new_token =
            JsSyntaxToken::new_detached(token.kind(), &num.format(ctx.options().clone()), [], []);
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

#[derive(Debug, Clone, Serialize, Deserialize, Deserializable, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseNumericSeparatorsOptions {
    /// Binary numeric literal options.
    #[serde(default)]
    pub binary: BaseOptions,
    /// Octal numeric literal options.
    #[serde(default)]
    pub octal: BaseOptions,
    /// Decimal numeric literal options.
    #[serde(default = "decimal_default")]
    pub decimal: BaseOptions,
    /// Hexadecimal numeric literal options.
    #[serde(default = "hexadecimal_default")]
    pub hexadecimal: BaseOptions,
}

impl Default for UseNumericSeparatorsOptions {
    fn default() -> Self {
        Self {
            binary: BaseOptions::default(),
            octal: BaseOptions::default(),
            decimal: decimal_default(),
            hexadecimal: hexadecimal_default(),
        }
    }
}

fn decimal_default() -> BaseOptions {
    BaseOptions {
        min_digits: 5,
        group_length: 3,
    }
}

fn hexadecimal_default() -> BaseOptions {
    BaseOptions {
        min_digits: 0,
        group_length: 2,
    }
}

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BaseOptions {
    /// The minimum number of digits before a separator is required.
    pub min_digits: usize,
    /// The size of each group of digits.
    pub group_length: usize,
}

impl Default for BaseOptions {
    fn default() -> Self {
        Self {
            min_digits: 0,
            group_length: 4,
        }
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
fn add_separators_from_right(num: &str, group_length: usize) -> String {
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

/// Add chunk separators to a number string, starting from the left. Used for fractional parts.
/// The "uneven" chunk is added to the right of the last separator.
/// 12345654321 -> 123_456_543_21
fn add_separators_from_left(num: &str, group_length: usize) -> String {
    num.chars()
        .collect::<Vec<_>>()
        .chunks(group_length)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("_")
}

#[derive(Debug)]
struct NumericLiteral {
    sign: Option<char>,
    radix: u8,
    number: String,
    fraction: Option<String>,
    exponent: Option<FractionalExponent>,
}

#[derive(Debug)]
struct FractionalExponent {
    e: char,
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
        let (sign, num) = split_into_sign_and_number(raw);
        let (radix, num) = split_into_radix_and_number(num);

        let (number, fractional) = num.split_once('.').unwrap_or((&num, ""));

        let (fraction, exponent) =
            fractional
                .split_once(['e', 'E'])
                .map_or((fractional, None), |(fraction, exponent)| {
                    let (sign, exponent) = split_into_sign_and_number(exponent);
                    (
                        fraction,
                        Some(FractionalExponent {
                            exponent: exponent.to_owned(),
                            sign,
                            e: if fractional.contains('e') { 'e' } else { 'E' },
                        }),
                    )
                });

        NumericLiteral {
            sign,
            radix,
            number: number.to_owned(),
            fraction: if fraction.is_empty() {
                None
            } else {
                Some(fraction.to_owned())
            },
            exponent,
        }
    }

    fn format(&self, options: UseNumericSeparatorsOptions) -> String {
        let BaseOptions {
            min_digits,
            group_length,
        } = match self.radix {
            2 => options.binary,
            8 => options.octal,
            10 => options.decimal,
            16 => options.hexadecimal,
            _ => unreachable!(),
        };

        let number = if self.number.len() < min_digits {
            self.number.clone()
        } else {
            add_separators_from_right(&self.number, group_length)
        };

        let fraction = if let Some(fraction) = &self.fraction {
            format!(
                ".{}",
                if fraction.len() < min_digits {
                    fraction.to_owned()
                } else {
                    add_separators_from_left(fraction, group_length)
                },
            )
        } else {
            String::new()
        };

        let exponent = if let Some(exp) = &self.exponent {
            format!(
                "{}{}{}",
                exp.e,
                exp.sign.unwrap_or_empty(),
                add_separators_from_right(&exp.exponent, group_length)
            )
        } else {
            String::new()
        };

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
